use crate::utils::builder::hash_state::ApplicationState;
use auto_di::singleton;
use dashmap::DashMap;
use sqlx::SqlitePool;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, Semaphore};

type ServerKey = Option<i64>;

pub struct BuilderQueue {
    pub(super) db: Arc<SqlitePool>,
    pub(super) application_state: Arc<ApplicationState>,
    slots: DashMap<ServerKey, Arc<Semaphore>>,
    per_server_limit: usize,
    notify: Notify,
    started: AtomicBool,
}

#[singleton]
impl BuilderQueue {
    pub fn new(db: Arc<SqlitePool>, application_state: Arc<ApplicationState>) -> Self {
        let per_server_limit = std::env::var("DEPLOYMENT_PER_SERVER_CONCURRENCY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1usize)
            .max(1);

        Self {
            db,
            application_state,
            slots: DashMap::new(),
            per_server_limit,
            notify: Notify::new(),
            started: AtomicBool::new(false),
        }
    }

    pub async fn start(self: &Arc<Self>) -> Result<(), String> {
        if self.started.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        self.recover_stale_deployments().await;
        let q = Arc::clone(self);
        tokio::spawn(async move {
            q.retry_stale_remote_cleanup().await;
        });

        let q = Arc::clone(self);
        tokio::spawn(async move {
            q.dispatch_loop().await;
        });

        let q = Arc::clone(self);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30 * 60));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                interval.tick().await;
                tracing::debug!("builder queue: 30-min reconciler tick");
                q.notify();
            }
        });

        tracing::info!(
            per_server_limit = self.per_server_limit,
            "builder queue started"
        );
        self.notify();
        Ok(())
    }

    pub fn notify(&self) {
        self.notify.notify_one();
    }

    pub async fn ensure_capacity(&self) -> sqlx::Result<()> {
        let max_size: i64 = std::env::var("DEPLOYMENT_QUEUE_MAX_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100)
            .max(1);

        let queued = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM deployments WHERE status = 'QUEUED'",
        )
        .fetch_one(self.db.as_ref())
        .await?;

        if queued >= max_size {
            return Err(sqlx::Error::Protocol(format!(
                "deployment queue is full ({queued}/{max_size}); try again later"
            )));
        }
        Ok(())
    }

    pub async fn cancel_queued_application(&self, application_id: i64) -> sqlx::Result<bool> {
        let ids: Vec<i64> = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM deployments WHERE application_id = ? AND status = 'QUEUED'",
        )
        .bind(application_id)
        .fetch_all(self.db.as_ref())
        .await?;

        let rows = sqlx::query(
            "UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE application_id = ? AND status = 'QUEUED'",
        )
        .bind(application_id)
        .execute(self.db.as_ref())
        .await?;

        if rows.rows_affected() > 0 {
            for id in ids {
                if let Ok(mut log) = super::deployment_log::DeploymentLog::open(id).await {
                    let _ = log.write_line("[CANCELLED] deployment cancelled before worker started").await;
                }
            }
        }
        Ok(rows.rows_affected() > 0)
    }

    pub async fn cancel_queued_compose(&self, compose_id: i64) -> sqlx::Result<bool> {
        let ids: Vec<i64> = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM deployments WHERE compose_id = ? AND status = 'QUEUED'",
        )
        .bind(compose_id)
        .fetch_all(self.db.as_ref())
        .await?;

        let rows = sqlx::query(
            "UPDATE deployments
             SET status        = 'CANCELLED',
                 state         = 'CANCELLED',
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE compose_id = ? AND status = 'QUEUED'",
        )
        .bind(compose_id)
        .execute(self.db.as_ref())
        .await?;

        if rows.rows_affected() > 0 {
            for id in ids {
                if let Ok(mut log) = super::deployment_log::DeploymentLog::open(id).await {
                    let _ = log.write_line("[CANCELLED] deployment cancelled before worker started").await;
                }
            }
        }
        Ok(rows.rows_affected() > 0)
    }

    fn semaphore_for(&self, server_id: ServerKey) -> Arc<Semaphore> {
        if let Some(entry) = self.slots.get(&server_id) {
            return Arc::clone(entry.value());
        }
        let sem = Arc::new(Semaphore::new(self.per_server_limit));
        self.slots.insert(server_id, Arc::clone(&sem));
        sem
    }


    async fn recover_stale_deployments(&self) {
        self.cleanup_stale_remote_jobs().await;

        if let Err(e) = sqlx::query(
            "UPDATE deployments
             SET status        = 'ERROR',
                 state         = 'RECOVERED_AFTER_RESTART',
                 error_message = COALESCE(error_message, 'server restarted while deployment was running'),
                 finished_at   = strftime('%s', 'now'),
                 last_state_at = strftime('%s', 'now')
             WHERE status = 'RUNNING'",
        )
        .execute(self.db.as_ref())
        .await
        {
            tracing::error!(error = %e, "builder queue: could not recover stale deployments");
        }

        if let Err(e) = sqlx::query(
            "UPDATE applications SET app_status = 'ERROR'
             WHERE app_status = 'RUNNING'
             AND id IN (
                 SELECT application_id FROM deployments
                 WHERE state = 'RECOVERED_AFTER_RESTART' AND application_id IS NOT NULL
             )",
        )
        .execute(self.db.as_ref())
        .await
        {
            tracing::error!(error = %e, "builder queue: could not recover stale application statuses");
        }

        if let Err(e) = sqlx::query(
            "UPDATE compose_projects SET compose_status = 'ERROR'
             WHERE compose_status = 'RUNNING'
             AND id IN (
                 SELECT compose_id FROM deployments
                 WHERE state = 'RECOVERED_AFTER_RESTART' AND compose_id IS NOT NULL
             )",
        )
        .execute(self.db.as_ref())
        .await
        {
            tracing::error!(error = %e, "builder queue: could not recover stale compose statuses");
        }
    }

    async fn cleanup_stale_remote_jobs(&self) {
        let rows = match sqlx::query_as::<_, (i64, i64, String)>(
            "SELECT id, server_id, pid FROM deployments
             WHERE status = 'RUNNING' AND server_id IS NOT NULL AND pid IS NOT NULL",
        )
        .fetch_all(self.db.as_ref())
        .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load stale remote jobs");
                return;
            }
        };

        for (deployment_id, server_id, pid_file) in rows {
            self.kill_remote_pid(deployment_id, server_id, &pid_file, "startup").await;
        }
    }

    async fn retry_stale_remote_cleanup(self: Arc<Self>) {
        for attempt in 1..=20 {
            let rows = match sqlx::query_as::<_, (i64, i64, String)>(
                "SELECT id, server_id, pid FROM deployments
                 WHERE status = 'ERROR' AND state = 'RECOVERED_AFTER_RESTART'
                 AND server_id IS NOT NULL AND pid IS NOT NULL",
            )
            .fetch_all(self.db.as_ref())
            .await
            {
                Ok(rows) => rows,
                Err(e) => {
                    tracing::error!(error = %e, "builder queue: could not load recovered remote jobs");
                    return;
                }
            };

            if rows.is_empty() {
                return;
            }

            let mut pending = 0usize;
            for (deployment_id, server_id, pid_file) in rows {
                let killed = self
                    .kill_remote_pid(deployment_id, server_id, &pid_file, &format!("retry-{attempt}"))
                    .await;
                if !killed {
                    pending += 1;
                }
            }

            if pending == 0 {
                return;
            }
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    }

    async fn kill_remote_pid(
        &self,
        deployment_id: i64,
        server_id: i64,
        pid_file: &str,
        stage: &str,
    ) -> bool {
        use crate::services::application::remote::remote_executor;
        match remote_executor(self.db.as_ref(), server_id).await {
            Ok(executor) => match executor.kill_pid_file(pid_file).await {
                Ok(()) => {
                    sqlx::query("UPDATE deployments SET pid = NULL WHERE id = ?")
                        .bind(deployment_id)
                        .execute(self.db.as_ref())
                        .await
                        .ok();
                    tracing::warn!(deployment_id, server_id, pid_file, stage,
                        "builder queue: killed stale remote deployment");
                    true
                }
                Err(e) => {
                    tracing::warn!(deployment_id, server_id, pid_file, stage,
                        error = %e, "builder queue: failed to kill stale remote deployment");
                    false
                }
            },
            Err(e) => {
                tracing::warn!(deployment_id, server_id, pid_file, stage,
                    error = %e, "builder queue: could not create remote executor for stale cleanup");
                false
            }
        }
    }

    // ------------------------------------------------------------------ //
    //  Internal dispatch                                                   //
    // ------------------------------------------------------------------ //

    async fn dispatch_loop(self: Arc<Self>) {
        loop {
            self.notify.notified().await;
            self.dispatch_available().await;
        }
    }

    async fn dispatch_available(self: &Arc<Self>) {
        // Load all QUEUED deployments grouped by server_id.
        // For each server, try to fill free slots.
        let queued = match sqlx::query_as::<_, (i64, Option<i64>)>(
            "SELECT id, server_id FROM deployments
             WHERE status = 'QUEUED'
             ORDER BY created_at ASC, id ASC",
        )
        .fetch_all(self.db.as_ref())
        .await
        {
            Ok(rows) => rows,
            Err(e) => {
                tracing::error!(error = %e, "builder queue: could not load queued deployments");
                return;
            }
        };

        for (deployment_id, server_id) in queued {
            let sem = self.semaphore_for(server_id);

            // Non-blocking — skip if this server is at capacity.
            let Ok(permit) = sem.try_acquire_owned() else {
                continue;
            };

            // Atomically claim this specific deployment.
            let row = sqlx::query(
                r#"UPDATE deployments
                   SET status        = 'RUNNING',
                       state         = 'QUEUE',
                       started_at    = COALESCE(started_at, strftime('%s', 'now')),
                       last_state_at = strftime('%s', 'now')
                   WHERE id = ? AND status = 'QUEUED'
                   RETURNING id, application_id, compose_id, operation"#,
            )
            .bind(deployment_id)
            .fetch_optional(self.db.as_ref())
            .await;

            match row {
                Ok(Some(row)) => {
                    use sqlx::Row;
                    let dep_id: i64 = row.get("id");
                    let application_id: Option<i64> = row.get("application_id");
                    let compose_id: Option<i64> = row.get("compose_id");
                    let operation: String = row
                        .get::<Option<String>, _>("operation")
                        .unwrap_or_else(|| "deploy".into());

                    let db = Arc::clone(&self.db);
                    let state = Arc::clone(&self.application_state);
                    let q = Arc::clone(self);

                    tokio::spawn(async move {
                        super::common::process(
                            db,
                            state,
                            dep_id,
                            application_id,
                            compose_id,
                            operation,
                        )
                        .await;
                        // Free the per-server slot, then wake dispatcher.
                        drop(permit);
                        q.notify();
                    });
                }
                Ok(None) => {
                    // Already claimed by someone else — release permit.
                    drop(permit);
                }
                Err(e) => {
                    tracing::error!(deployment_id, error = %e,
                        "builder queue: failed to claim deployment");
                    drop(permit);
                }
            }
        }
    }
}
