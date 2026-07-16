use std::sync::Arc;

use auto_di::resolve;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    core::config::Config,
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
        cgroup::{CgroupBuilder, MemoryLimit, CpuLimit},
    },
};

fn parse_memory_limit(s: &str) -> Option<MemoryLimit> {
    let s = s.trim().to_lowercase();
    if s == "max" {
        return Some(MemoryLimit::Max);
    }
    let num_str = s.chars().take_while(|c| c.is_ascii_digit()).collect::<String>();
    let suffix = s.chars().skip_while(|c| c.is_ascii_digit()).collect::<String>();
    let val: u64 = num_str.parse().ok()?;
    
    match suffix.trim() {
        "k" | "kb" => Some(MemoryLimit::KB(val)),
        "m" | "mb" => Some(MemoryLimit::MB(val)),
        "g" | "gb" => Some(MemoryLimit::GB(val)),
        "b" | "" => Some(MemoryLimit::B(val)),
        _ => None,
    }
}

fn parse_cpu_limit(s: &str) -> Option<CpuLimit> {
    let s = s.trim().to_lowercase();
    if s == "max" {
        return Some(CpuLimit::Max);
    }
    if let Ok(cores) = s.parse::<f32>() {
        return Some(CpuLimit::Cores(cores));
    }
    None
}

async fn get_total_memory_kb(executor: &CommandExecutor) -> Option<u64> {
    if let Ok(res) = executor.run("cat", &["/proc/meminfo"]).await {
        for line in res.stdout.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<u64>().ok();
                }
            }
        }
    }
    None
}

async fn get_cpu_cores(executor: &CommandExecutor) -> Option<f32> {
    if let Ok(res) = executor.run("nproc", &[] as &[&str]).await {
        if let Ok(cores) = res.stdout.trim().parse::<f32>() {
            return Some(cores);
        }
    }
    if let Ok(res) = executor.run("cat", &["/proc/cpuinfo"]).await {
        let mut count = 0;
        for line in res.stdout.lines() {
            if line.starts_with("processor") {
                count += 1;
            }
        }
        if count > 0 {
            return Some(count as f32);
        }
    }
    None
}

impl BuilderQueue {
    pub(crate) async fn execute_operation_app(
        db: Arc<SqlitePool>,
        application_id: i64,
        deployment_id: i64,
        _operation: ApplicationOperation,
    ) -> Result<(), BuilderError> {
        let spec_adapter = resolve::<ApplicationSpecAdapter>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve ApplicationSpecAdapter: {e}")))?;
        let spec = spec_adapter
            .load(application_id)
            .await
            .map_err(|e| BuilderError::Execution(format!("could not load deployment config: {e}")))?;

        let app_repo = resolve::<crate::repository::ApplicationRepository>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve ApplicationRepository: {e}")))?;
        let (environment_id, project_id, server_id) = app_repo.get_deployment_context(application_id)
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

        let config = resolve::<Config>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve application config: {e}")))?;

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                let dep_repo = resolve::<crate::repository::DeploymentRepository>()
                    .await
                    .map_err(|e| BuilderError::Execution(format!("could not resolve DeploymentRepository: {e}")))?;
                dep_repo.set_pid(deployment_id, &pid_file)
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

        // Determine the build cgroup limits (SQLite -> dynamic resource fallbacks -> config values)
        let mut mem_limit_str = None;
        let mut cpu_limit_str = None;

        if let Some(sid) = server_id {
            let server_repo = resolve::<crate::repository::ServerRepository>()
                .await
                .map_err(|e| BuilderError::Execution(format!("could not resolve ServerRepository: {e}")))?;
            if let Ok(Some(server)) = server_repo.get_by_id(sid).await {
                if let Some(val) = server.build_memory_limit {
                    if !val.trim().is_empty() {
                        mem_limit_str = Some(val);
                    }
                }
                if let Some(val) = server.build_cpu_limit {
                    if !val.trim().is_empty() {
                        cpu_limit_str = Some(val);
                    }
                }
            }
        }

        let mut mem_limit = mem_limit_str.as_deref().and_then(parse_memory_limit);
        let mut cpu_limit = cpu_limit_str.as_deref().and_then(parse_cpu_limit);

        // Fallback: Dynamic system resource parsing (Target RAM / 2, Target CPU / 2)
        if mem_limit.is_none() {
            if let Some(total_kb) = get_total_memory_kb(&executor).await {
                mem_limit = Some(MemoryLimit::KB(total_kb / 2));
            }
        }
        if cpu_limit.is_none() {
            if let Some(total_cores) = get_cpu_cores(&executor).await {
                let half_cores = (total_cores / 2.0).max(1.0);
                cpu_limit = Some(CpuLimit::Cores(half_cores));
            }
        }

        // Secondary Fallback: Application config defaults
        if mem_limit.is_none() {
            mem_limit = parse_memory_limit(&config.build_memory_limit);
        }
        if cpu_limit.is_none() {
            cpu_limit = parse_cpu_limit(&config.build_cpu_limit);
        }

        // Apply/ensure the shared build cgroup is present with resolved limits
        let mut cgroup = None;
        if mem_limit.is_some() || cpu_limit.is_some() {
            let mut builder = CgroupBuilder::new("rustploy-build", executor.clone());
            if let Some(mem) = mem_limit {
                builder = builder.memory(mem);
            }
            if let Some(cpu) = cpu_limit {
                builder = builder.cpu(cpu);
            }
            let cg = builder.build();
            if let Err(e) = cg.apply().await {
                tracing::warn!(error = %e, "Failed to apply/ensure build cgroup, proceeding without limits");
            } else {
                cgroup = Some(cg);
            }
        }

        let (events_tx, events_rx) = mpsc::channel(6);
        tokio::spawn(super::deployment_log::record_builder_events(db.clone(), deployment_id, events_rx, "app"));

        let events_tx_clone = events_tx.clone();
        let cgroup_clone = cgroup.clone();
        let build_future = async move {
            let mut builder = ApplicationBuilder::new(executor)
                .with_state(state, app_key)
                .with_events(events_tx);
                
            if let Some(cg) = cgroup_clone {
                builder = builder.with_cgroup(cg);
            }
            
            builder
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
