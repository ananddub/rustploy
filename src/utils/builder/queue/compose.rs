use std::sync::Arc;

use auto_di::resolve;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compose::{
        ComposeOperation,
        remote::{deployment_pid_file, remote_executor},
    },
    utils::{
        builder::{
            compose::{adapter::ComposeSpecAdapter, ComposeBuilder},
            custom_type::IdType,
            hash_state::ApplicationState,
            queue::{common, queue::BuilderQueue},
            spec::BuilderEvent,
        },
        exec::{CommandExecutor, LocalExecutor},
        builder::errors::BuilderError,
    },
};

impl BuilderQueue {
    pub(crate) async fn execute_operation_compose(
        db: Arc<SqlitePool>,
        compose_id: i64,
        deployment_id: i64,
        operation: ComposeOperation,
    ) -> Result<(), BuilderError> {
        let spec = ComposeSpecAdapter::new(db.clone())
            .load(compose_id)
            .await
            .map_err(|e| BuilderError::Execution(format!("could not load compose config: {e}")))?;

        let (environment_id, project_id, server_id) =
            sqlx::query_as::<_, (i64, i64, Option<i64>)>(
                r#"SELECT c.environment_id, e.project_id, c.server_id
                   FROM compose_projects c
                   JOIN environments e ON e.id = c.environment_id
                   WHERE c.id = ?"#,
            )
            .bind(compose_id)
            .fetch_one(db.as_ref())
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve compose context: {e}")))?;

        let compose_key = IdType::ComposeId(compose_id);
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve application state: {e}")))?;
        state.reset_default(compose_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(compose_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                sqlx::query("UPDATE deployments SET pid = ? WHERE id = ?")
                    .bind(&pid_file)
                    .bind(deployment_id)
                    .execute(db.as_ref())
                    .await
                    .map_err(|e| BuilderError::Execution(format!("could not persist remote compose pid file: {e}")))?;
                CommandExecutor::Remote(
                    remote_executor(db.as_ref(), sid)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))?
                        .with_job_pid_file(pid_file),
                )
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let (events_tx, events_rx) = mpsc::channel(64);
        tokio::spawn(record_builder_events(db.clone(), deployment_id, events_rx));

        let builder = ComposeBuilder::new(executor)
            .with_state(state, compose_key)
            .with_events(events_tx);

        match operation {
            ComposeOperation::Stop => builder.stop(&spec).await.map_err(|e| BuilderError::Execution(e.to_string())),
            _ => builder
                .deploy(&spec, &cancel)
                .await
                .map(|_| ())
                .map_err(|e| BuilderError::Execution(e.to_string())),
        }
    }
}

async fn record_builder_events(
    db: Arc<SqlitePool>,
    deployment_id: i64,
    mut events: mpsc::Receiver<BuilderEvent>,
) {
    while let Some(event) = events.recv().await {
        if let BuilderEvent::Message(message) = &event {
            tracing::info!(deployment_id, message = %message, "compose deployment message");
            continue;
        }
        let state = common::builder_event_state(&event);
        let message = match &event {
            BuilderEvent::Failed(e) => Some(e.as_str()),
            _ => None,
        };
        if let Err(e) = sqlx::query(
            "UPDATE deployments
             SET state         = ?,
                 error_message = COALESCE(?, error_message),
                 last_state_at = strftime('%s', 'now')
             WHERE id = ? AND status = 'RUNNING'",
        )
        .bind(state)
        .bind(message)
        .bind(deployment_id)
        .execute(db.as_ref())
        .await
        {
            tracing::error!(deployment_id, error = %e, "could not persist compose builder event");
        }
    }
}
