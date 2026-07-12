use std::{
    collections::{HashMap, HashSet},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use auto_di::singleton;
use dashmap::{DashMap, DashSet};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use super::schedule::ScheduleService;

const REFRESH_CRON: &str = "0/30 * * * * *";

pub struct ScheduleRunner {
    service: Arc<ScheduleService>,
    scheduler: Mutex<Option<Arc<JobScheduler>>>,
    jobs: DashMap<i64, RegisteredScheduleJob>,
    in_flight: DashSet<i64>,
    started: AtomicBool,
}

#[derive(Debug, Clone)]
struct RegisteredScheduleJob {
    job_id: Uuid,
    cron_expression: String,
    enabled: i64,
}

#[singleton]
impl ScheduleRunner {
    fn new(service: Arc<ScheduleService>) -> Self {
        Self {
            service,
            scheduler: Mutex::new(None),
            jobs: DashMap::new(),
            in_flight: DashSet::new(),
            started: AtomicBool::new(false),
        }
    }

    pub async fn start(self: &Arc<Self>) -> Result<(), String> {
        if self.started.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        let scheduler = Arc::new(
            JobScheduler::new()
                .await
                .map_err(|error| format!("could not create schedule runner: {error}"))?,
        );

        let runner = Arc::clone(self);
        scheduler
            .add(
                Job::new_async(REFRESH_CRON, move |_job_id, _lock| {
                    let runner = Arc::clone(&runner);
                    Box::pin(async move {
                        if let Err(error) = runner.refresh_jobs().await {
                            tracing::error!(error = %error, "schedule refresh failed");
                        }
                    })
                })
                .map_err(|error| format!("could not create schedule refresh job: {error}"))?,
            )
            .await
            .map_err(|error| format!("could not register schedule refresh job: {error}"))?;

        {
            let mut guard = self.scheduler.lock().await;
            *guard = Some(Arc::clone(&scheduler));
        }

        self.refresh_jobs().await?;
        scheduler
            .start()
            .await
            .map_err(|error| format!("could not start schedule runner: {error}"))?;
        tracing::info!("schedule runner started");
        Ok(())
    }

    pub async fn refresh_jobs(&self) -> Result<(), String> {
        let scheduler = self
            .scheduler
            .lock()
            .await
            .clone()
            .ok_or("schedule runner is not started")?;
        let schedules = self
            .service
            .list_enabled()
            .await
            .map_err(|error| format!("could not load enabled schedules: {error}"))?;
        let enabled_ids = schedules
            .iter()
            .filter_map(|schedule| schedule.id)
            .collect::<HashSet<_>>();

        let stale_jobs = self
            .jobs
            .iter()
            .filter_map(|entry| {
                let schedule_id = *entry.key();
                if enabled_ids.contains(&schedule_id) {
                    None
                } else {
                    Some((schedule_id, entry.value().job_id))
                }
            })
            .collect::<Vec<_>>();
        for (schedule_id, job_id) in stale_jobs {
            self.remove_job(&scheduler, schedule_id, job_id).await;
        }

        let current = self
            .jobs
            .iter()
            .map(|entry| (*entry.key(), entry.value().clone()))
            .collect::<HashMap<_, _>>();

        for schedule in schedules {
            let Some(schedule_id) = schedule.id else {
                continue;
            };
            let cron_expression = normalize_cron_expression(&schedule.cron_expression);
            let should_replace = current.get(&schedule_id).is_none_or(|registered| {
                registered.cron_expression != cron_expression
                    || registered.enabled != schedule.enabled
            });
            if !should_replace {
                continue;
            }
            if let Some(registered) = current.get(&schedule_id) {
                self.remove_job(&scheduler, schedule_id, registered.job_id)
                    .await;
            }

            let service = Arc::clone(&self.service);
            let in_flight = self.in_flight.clone();
            let job = Job::new_async(cron_expression.as_str(), move |_job_id, _lock| {
                let service = Arc::clone(&service);
                let in_flight = in_flight.clone();
                Box::pin(async move {
                    if !in_flight.insert(schedule_id) {
                        tracing::warn!(schedule_id, "schedule skipped because previous run is still active");
                        return;
                    }
                    let result = service.run_now(schedule_id).await;
                    in_flight.remove(&schedule_id);
                    match result {
                        Ok(result) => tracing::info!(
                            schedule_id,
                            action = %result.action,
                            deployment_id = ?result.deployment_id,
                            "scheduled job executed"
                        ),
                        Err(error) => tracing::error!(
                            schedule_id,
                            error = %error,
                            "scheduled job failed"
                        ),
                    }
                })
            })
            .map_err(|error| {
                format!(
                    "invalid cron expression for schedule {schedule_id} ({cron_expression}): {error}"
                )
            })?;
            let job_id = job.guid();
            scheduler
                .add(job)
                .await
                .map_err(|error| format!("could not register schedule {schedule_id}: {error}"))?;
            self.jobs.insert(
                schedule_id,
                RegisteredScheduleJob {
                    job_id,
                    cron_expression,
                    enabled: schedule.enabled,
                },
            );
            tracing::info!(schedule_id, "schedule job registered");
        }

        Ok(())
    }

    async fn remove_job(&self, scheduler: &JobScheduler, schedule_id: i64, job_id: Uuid) {
        if let Err(error) = scheduler.remove(&job_id).await {
            tracing::warn!(schedule_id, job_id = %job_id, error = %error, "could not remove schedule job");
        }
        self.jobs.remove(&schedule_id);
        self.in_flight.remove(&schedule_id);
    }
}

fn normalize_cron_expression(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.split_whitespace().count() == 5 {
        format!("0 {trimmed}")
    } else {
        trimmed.into()
    }
}
