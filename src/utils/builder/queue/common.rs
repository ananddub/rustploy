use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    services::{
        application::ApplicationOperation,
        compose::ComposeOperation,
    },
    utils::builder::{
        custom_type::{DeployState, IdType},
        hash_state::ApplicationState,
        queue::queue::BuilderQueue,
        spec::BuilderEvent,
    },
};

pub async fn process(
    db: Arc<SqlitePool>,
    application_state: Arc<ApplicationState>,
    deployment_id: i64,
    application_id: Option<i64>,
    compose_id: Option<i64>,
    operation: String,
) {

    tracing::info!(
        deployment_id,
        operation = %operation,
        application_id,
        compose_id,
        "builder queue: job started"
    );

    let result = match (application_id, compose_id) {
        (Some(app_id), None) => {
            let op = parse_application_operation(&operation);
            BuilderQueue::execute_operation_app(db.clone(), app_id, deployment_id, op).await
        }
        (None, Some(cmp_id)) => {
            let op = parse_compose_operation(&operation);
            BuilderQueue::execute_operation_compose(db.clone(), cmp_id, deployment_id, op).await
        }
        _ => Err(crate::utils::builder::errors::BuilderError::Execution(format!(
            "deployment {deployment_id} must have exactly one of application_id or compose_id"
        ))),
    };

    let final_status = match &result {
        Ok(()) => "DONE",
        Err(e) if is_cancelled_error(&e.to_string()) => "CANCELLED",
        Err(_) => "ERROR",
    };

    let error_message = result.err().map(|e| e.to_string());

    if let Err(e) = sqlx::query(
        "UPDATE deployments
         SET status        = ?,
             state         = ?,
             error_message = COALESCE(?, error_message),
             finished_at   = strftime('%s', 'now'),
             last_state_at = strftime('%s', 'now')
         WHERE id = ?",
    )
    .bind(final_status)
    .bind(final_status)
    .bind(error_message.as_deref())
    .bind(deployment_id)
    .execute(db.as_ref())
    .await
    {
        tracing::error!(deployment_id, error = %e, "builder queue: could not persist final deployment status");
    }

    let target_status = if final_status == "DONE" { "DONE" } else { "ERROR" };
    if let Some(app_id) = application_id {
        if let Err(e) = sqlx::query("UPDATE applications SET app_status = ? WHERE id = ?")
            .bind(target_status)
            .bind(app_id)
            .execute(db.as_ref())
            .await
        {
            tracing::error!(deployment_id, app_id, error = %e, "builder queue: could not persist application status");
        }
        application_state.remove_state(IdType::AppId(app_id));
    }

    if let Some(cmp_id) = compose_id {
        if let Err(e) = sqlx::query("UPDATE compose_projects SET compose_status = ? WHERE id = ?")
            .bind(target_status)
            .bind(cmp_id)
            .execute(db.as_ref())
            .await
        {
            tracing::error!(deployment_id, cmp_id, error = %e, "builder queue: could not persist compose status");
        }
        application_state.remove_state(IdType::ComposeId(cmp_id));
    }

    tracing::info!(
        deployment_id,
        status = final_status,
        "builder queue: job finished"
    );
}

pub fn builder_event_state(event: &BuilderEvent) -> &'static str {
    match event {
        BuilderEvent::Preparing => "PREPARING",
        BuilderEvent::SourceReady => "SOURCE_READY",
        BuilderEvent::Building => "BUILDING",
        BuilderEvent::ImageReady => "IMAGE_READY",
        BuilderEvent::Deploying => "DEPLOYING",
        BuilderEvent::Routing => "ROUTING",
        BuilderEvent::HealthCheck => "HEALTH_CHECK",
        BuilderEvent::Deployed => "DEPLOYED",
        BuilderEvent::Cancelled => "CANCELLED",
        BuilderEvent::Message(_) => "MESSAGE",
        BuilderEvent::Failed(_) => "FAILED",
        BuilderEvent::RecoverAfterRestart => "RECOVER_AFTER_RESTART",
    }
}


pub fn builder_event_state_opt(event: &BuilderEvent) -> Option<DeployState> {
    match event {
        BuilderEvent::Preparing => Some(DeployState::Preparing),
        BuilderEvent::SourceReady => Some(DeployState::GitSuccess),
        BuilderEvent::Building => Some(DeployState::Building),
        BuilderEvent::ImageReady => Some(DeployState::BuildSuccess),
        BuilderEvent::Deploying | BuilderEvent::Routing => Some(DeployState::Deploying),
        BuilderEvent::HealthCheck => Some(DeployState::HealthCheck),
        BuilderEvent::Deployed => Some(DeployState::Deployed),
        BuilderEvent::Cancelled => Some(DeployState::StoppedByUser),
        BuilderEvent::Failed(error) => Some(DeployState::Failed(error.clone())),
        BuilderEvent::RecoverAfterRestart => Some(DeployState::RecoverAfterRestart),
        BuilderEvent::Message(_) => None,
    }
}

pub fn is_cancelled_error(error: &str) -> bool {
    error.to_ascii_lowercase().contains("cancel")
}

fn parse_application_operation(value: &str) -> ApplicationOperation {
    match value {
        "redeploy" => ApplicationOperation::Redeploy,
        "rebuild"  => ApplicationOperation::Rebuild,
        "reload"   => ApplicationOperation::Reload,
        "start"    => ApplicationOperation::Start,
        _          => ApplicationOperation::Deploy,
    }
}

fn parse_compose_operation(value: &str) -> ComposeOperation {
    match value {
        "redeploy" => ComposeOperation::Redeploy,
        "reload"   => ComposeOperation::Reload,
        "start"    => ComposeOperation::Start,
        "stop"     => ComposeOperation::Stop,
        _          => ComposeOperation::Deploy,
    }
}
