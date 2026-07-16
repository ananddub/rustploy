use std::sync::Arc;

use auto_di::resolve;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    core::config::Config,
    services::{
        database::{DatabaseOperation, DatabaseKind},
        compose::remote::{deployment_pid_file, remote_executor},
    },
    utils::{
        builder::{
            database::builder::DatabaseBuilder,
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
    pub(crate) async fn execute_operation_db(
        db_pool: Arc<SqlitePool>,
        database_id: i64,
        database_kind: String,
        deployment_id: i64,
        operation: DatabaseOperation,
    ) -> Result<(), BuilderError> {
        let kind = std::str::FromStr::from_str(&database_kind)
            .map_err(|_| BuilderError::Execution(format!("Invalid database kind: {}", database_kind)))?;

        let (server_id, environment_id, project_id, memory_limit_db, cpu_limit_db) = match kind {
            DatabaseKind::Postgres => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM postgres_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database postgres not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
            DatabaseKind::Mysql => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM mysql_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database mysql not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
            DatabaseKind::Mariadb => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM mariadb_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database mariadb not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
            DatabaseKind::Mongo => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM mongo_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database mongo not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
            DatabaseKind::Redis => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM redis_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database redis not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
            DatabaseKind::Libsql => {
                let r = sqlx::query!(
                    r#"SELECT d.server_id, d.environment_id, e.project_id, d.memory_limit, d.cpu_limit
                       FROM libsql_dbs d
                       JOIN environments e ON e.id = d.environment_id
                       WHERE d.id = ?"#,
                    database_id
                )
                .fetch_one(db_pool.as_ref())
                .await
                .map_err(|e| BuilderError::Execution(format!("Database libsql not found: {}", e)))?;
                (r.server_id, r.environment_id, r.project_id, r.memory_limit, r.cpu_limit)
            }
        };

        let db_key = IdType::DatabaseId(database_id);
        let state = resolve::<ApplicationState>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve application state: {e}")))?;
        state.reset_default(db_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(db_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let config = resolve::<Config>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve application config: {e}")))?;

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                sqlx::query("UPDATE deployments SET pid = ? WHERE id = ?")
                    .bind(&pid_file)
                    .bind(deployment_id)
                    .execute(db_pool.as_ref())
                    .await
                    .map_err(|e| BuilderError::Execution(format!("could not persist remote deployment pid file: {e}")))?;
                CommandExecutor::Remote(
                    remote_executor(db_pool.as_ref(), sid)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))?
                        .with_job_pid_file(pid_file),
                )
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let mut mem_limit_str = memory_limit_db;
        let mut cpu_limit_str = cpu_limit_db;

        if mem_limit_str.is_none() || cpu_limit_str.is_none() {
            if let Some(sid) = server_id {
                if let Ok(server) = sqlx::query_as::<_, crate::db::models::servers::Server>("SELECT * FROM servers WHERE id = ?")
                    .bind(sid)
                    .fetch_one(db_pool.as_ref())
                    .await
                {
                    if mem_limit_str.is_none() {
                        mem_limit_str = server.build_memory_limit;
                    }
                    if cpu_limit_str.is_none() {
                        cpu_limit_str = server.build_cpu_limit;
                    }
                }
            }
        }

        let mut mem_limit = mem_limit_str.as_deref().and_then(parse_memory_limit);
        let mut cpu_limit = cpu_limit_str.as_deref().and_then(parse_cpu_limit);

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

        if mem_limit.is_none() {
            mem_limit = parse_memory_limit(&config.build_memory_limit);
        }
        if cpu_limit.is_none() {
            cpu_limit = parse_cpu_limit(&config.build_cpu_limit);
        }

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
        tokio::spawn(super::deployment_log::record_builder_events(db_pool.clone(), deployment_id, events_rx, "db"));

        let events_tx_clone = events_tx.clone();
        let cgroup_clone = cgroup.clone();
        
        let build_future = async move {
            let mut builder = DatabaseBuilder::new(executor)
                .with_state(state, db_key)
                .with_events(events_tx);
                
            if let Some(cg) = cgroup_clone {
                builder = builder.with_cgroup(cg);
            }

            match operation {
                DatabaseOperation::Deploy | DatabaseOperation::Redeploy | DatabaseOperation::Reload | DatabaseOperation::Start => {
                    builder
                        .deploy(kind, database_id, db_pool.clone(), &cancel)
                        .await
                        .map(|_| ())
                        .map_err(|e| BuilderError::Execution(e.to_string()))
                }
                DatabaseOperation::Stop => {
                    let app_name = match kind {
                        DatabaseKind::Postgres => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM postgres_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                        DatabaseKind::Mysql => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM mysql_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                        DatabaseKind::Mariadb => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM mariadb_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                        DatabaseKind::Mongo => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM mongo_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                        DatabaseKind::Redis => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM redis_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                        DatabaseKind::Libsql => {
                            sqlx::query_scalar::<_, String>("SELECT app_name FROM libsql_dbs WHERE id = ?").bind(database_id).fetch_one(db_pool.as_ref()).await
                        }
                    }.map_err(|e| BuilderError::Execution(format!("Failed to query database app_name: {}", e)))?;

                    builder
                        .stop(&app_name, &cancel)
                        .await
                        .map(|_| ())
                        .map_err(|e| BuilderError::Execution(e.to_string()))
                }
            }
        };

        super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await
    }
}
