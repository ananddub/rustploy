use std::sync::Arc;

use auto_di::resolve;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::{
        application::ApplicationOperation,
        compose::remote::{deployment_pid_file, remote_executor},
    },
    utils::{
        builder::{
            adapter::ApplicationSpecAdapter,
            application::ApplicationBuilder,
            custom_type::IdType,
            hash_state::ApplicationState,
            queue::queue::BuilderQueue,
        },
        exec::{CommandExecutor, LocalExecutor},
        builder::errors::BuilderError,
    },
};

impl BuilderQueue {
    pub(crate) async fn execute_operation_app(
        db: Arc<SqlitePool>,
        application_id: i64,
        deployment_id: i64,
        _operation: ApplicationOperation,
    ) -> Result<(), BuilderError> {
        let spec = ApplicationSpecAdapter::new(db.clone())
            .load(application_id)
            .await
            .map_err(|e| BuilderError::Execution(format!("could not load deployment config: {e}")))?;

        let (environment_id, project_id, server_id) =
            sqlx::query_as::<_, (i64, i64, Option<i64>)>(
                r#"SELECT a.environment_id, e.project_id, a.server_id
                   FROM applications a
                   JOIN environments e ON e.id = a.environment_id
                   WHERE a.id = ?"#,
            )
            .bind(application_id)
            .fetch_one(db.as_ref())
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve deployment context: {e}")))?;

        let app_key = IdType::AppId(application_id);
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve application state: {e}")))?;
        state.reset_default(app_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(app_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                sqlx::query("UPDATE deployments SET pid = ? WHERE id = ?")
                    .bind(&pid_file)
                    .bind(deployment_id)
                    .execute(db.as_ref())
                    .await
                    .map_err(|e| BuilderError::Execution(format!("could not persist remote deployment pid file: {e}")))?;
                CommandExecutor::Remote(
                    remote_executor(db.as_ref(), sid)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))?
                        .with_job_pid_file(pid_file),
                )
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let (events_tx, events_rx) = mpsc::channel(6);
        tokio::spawn(super::deployment_log::record_builder_events(db.clone(), deployment_id, events_rx, "app"));

        let events_tx_clone = events_tx.clone();
        let build_future = async move {
            ApplicationBuilder::new(executor)
                .with_state(state, app_key)
                .with_events(events_tx)
                .deploy(&spec, &cancel)
                .await
                .map(|_| ())
                .map_err(|e| BuilderError::Execution(e.to_string()))
        };

        super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await
    }
}
