use std::sync::Arc;
use auto_di::singleton;
use sqlx::SqlitePool;
use tokio::sync::Semaphore;
use tokio_cron_scheduler::Job;
use crate::utils::builder::hash_state::ApplicationState;

pub struct BuilderQueue {
    db :Arc<SqlitePool>,
    application_state: Arc<ApplicationState>
}

#[singleton]
impl BuilderQueue {
    pub fn new(db: Arc<SqlitePool>, application_state: Arc<ApplicationState>) -> Self {
        Self {
            db,
            application_state,
        }
    }

    pub async fn cronjob_30_min(&self) {
        let scheduler = tokio_cron_scheduler::JobScheduler::new().await.unwrap();
        scheduler.add(
            Job::new_async( "0 */30 * * * *", |_uuid, _lock| {
                Box::pin(async move {
                    println!("Runs every 30 seconds");
                })
            }).unwrap(),
        ).await.unwrap();
    }

    pub fn update_queue() {}

    pub async fn push_app_queue(&self) {}
    pub async fn cancel_app_queue(&self) {}

    pub async fn push_compose_queue(&self) {}

    pub async fn cancel_compose_queue(&self) {}

}

