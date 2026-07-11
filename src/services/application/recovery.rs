use std::{sync::Arc, time::Duration};

use sqlx::SqlitePool;

use super::remote::remote_executor;

pub(super) fn spawn_recover_stale_deployments(db: Arc<SqlitePool>) {
    tokio::spawn(async move {
        cleanup_stale_remote_jobs(db.clone(), "application").await;
        if let Err(error) = sqlx::query(
            "UPDATE deployments SET status = 'ERROR', state = 'RECOVERED_AFTER_RESTART', error_message = COALESCE(error_message, 'server restarted while deployment was running'), finished_at = strftime('%s', 'now'), last_state_at = strftime('%s', 'now') WHERE status = 'RUNNING' AND application_id IS NOT NULL",
        )
        .execute(db.as_ref())
        .await
        {
            tracing::error!(error = %error, "could not recover stale application deployments");
        }
        if let Err(error) = sqlx::query(
            "UPDATE applications SET app_status = 'ERROR' WHERE app_status = 'RUNNING' AND id IN (SELECT application_id FROM deployments WHERE state = 'RECOVERED_AFTER_RESTART' AND application_id IS NOT NULL)",
        )
        .execute(db.as_ref())
        .await
        {
            tracing::error!(error = %error, "could not recover stale application statuses");
        }
        spawn_recovered_remote_cleanup_retry(db.clone(), "application");
    });
}

async fn cleanup_stale_remote_jobs(db: Arc<SqlitePool>, kind: &'static str) {
    let rows = match sqlx::query_as::<_, (i64, i64, String)>(
        "SELECT id, server_id, pid FROM deployments WHERE status = 'RUNNING' AND application_id IS NOT NULL AND server_id IS NOT NULL AND pid IS NOT NULL",
    )
    .fetch_all(db.as_ref())
    .await
    {
        Ok(rows) => rows,
        Err(error) => {
            tracing::error!(error = %error, "could not load stale application remote jobs");
            return;
        }
    };

    for (deployment_id, server_id, pid_file) in rows {
        match remote_executor(db.as_ref(), server_id).await {
            Ok(executor) => {
                if let Err(error) = executor.kill_pid_file(&pid_file).await {
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        error = %error,
                        "failed to cleanup stale remote deployment job after restart"
                    );
                } else {
                    clear_deployment_pid(db.clone(), deployment_id).await;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        kind,
                        "cleaned stale remote deployment job after restart"
                    );
                }
            }
            Err(error) => {
                tracing::warn!(
                    deployment_id,
                    server_id,
                    pid_file = %pid_file,
                    error = %error,
                    "could not create remote executor for stale deployment cleanup"
                );
            }
        }
    }
}

fn spawn_recovered_remote_cleanup_retry(db: Arc<SqlitePool>, kind: &'static str) {
    tokio::spawn(async move {
        for attempt in 1..=20 {
            let pending = cleanup_recovered_remote_jobs(db.clone(), kind, attempt).await;
            if pending == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

async fn cleanup_recovered_remote_jobs(
    db: Arc<SqlitePool>,
    kind: &'static str,
    attempt: usize,
) -> usize {
    let rows = match sqlx::query_as::<_, (i64, i64, String)>(
        "SELECT id, server_id, pid FROM deployments WHERE status = 'ERROR' AND state = 'RECOVERED_AFTER_RESTART' AND application_id IS NOT NULL AND server_id IS NOT NULL AND pid IS NOT NULL",
    )
    .fetch_all(db.as_ref())
    .await
    {
        Ok(rows) => rows,
        Err(error) => {
            tracing::error!(error = %error, "could not load recovered application remote jobs");
            return 0;
        }
    };

    let mut pending = 0;
    for (deployment_id, server_id, pid_file) in rows {
        match remote_executor(db.as_ref(), server_id).await {
            Ok(executor) => match executor.kill_pid_file(&pid_file).await {
                Ok(()) => {
                    clear_deployment_pid(db.clone(), deployment_id).await;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        kind,
                        attempt,
                        "cleaned recovered remote deployment job"
                    );
                }
                Err(error) => {
                    pending += 1;
                    tracing::warn!(
                        deployment_id,
                        server_id,
                        pid_file = %pid_file,
                        error = %error,
                        kind,
                        attempt,
                        "remote deployment cleanup retry failed"
                    );
                }
            },
            Err(error) => {
                pending += 1;
                tracing::warn!(
                    deployment_id,
                    server_id,
                    pid_file = %pid_file,
                    error = %error,
                    kind,
                    attempt,
                    "could not create remote executor for recovered deployment cleanup retry"
                );
            }
        }
    }
    pending
}

async fn clear_deployment_pid(db: Arc<SqlitePool>, deployment_id: i64) {
    if let Err(error) = sqlx::query("UPDATE deployments SET pid = NULL WHERE id = ?")
        .bind(deployment_id)
        .execute(db.as_ref())
        .await
    {
        tracing::error!(deployment_id, error = %error, "could not clear deployment pid file");
    }
}
