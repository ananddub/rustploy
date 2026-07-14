use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    api::dto::schedule::{CreateScheduleDto, PatchScheduleDto},
    db::models::schedules::Schedule,
    services::{
        application::{ApplicationOperation, ApplicationService},
        compose::{ComposeOperation, ComposeService, remote::remote_executor},
    },
    utils::{
        docker::DockerCli,
        exec::{CommandExecutor, LocalExecutor},
    },
};
use crate::services::compose::ComposeType;
use crate::services::schedule::types::{ScheduleAction, ScheduleType, ShellType};

#[derive(Debug, Clone)]
pub struct ScheduleRunResult {
    pub schedule: Schedule,
    pub action: String,
    pub deployment_id: Option<i64>,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub struct ScheduleService {
    db: Arc<SqlitePool>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
}

#[singleton]
impl ScheduleService {
    fn new(
        db: Arc<SqlitePool>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
    ) -> Self {
        Self {
            db,
            applications,
            compose,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Schedule> {
        select_schedule_by_id(self.db.as_ref(), id).await
    }

    pub async fn list_by_application(&self, application_id: i64) -> sqlx::Result<Vec<Schedule>> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE application_id = ?
               ORDER BY created_at DESC, id DESC"#,
            application_id,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<Schedule>> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE compose_id = ?
               ORDER BY created_at DESC, id DESC"#,
            compose_id,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_by_server(&self, server_id: i64) -> sqlx::Result<Vec<Schedule>> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE server_id = ?
               ORDER BY created_at DESC, id DESC"#,
            server_id,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> sqlx::Result<Vec<Schedule>> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC"#,
            organization_id,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn list_enabled(&self) -> sqlx::Result<Vec<Schedule>> {
        sqlx::query_as!(
            Schedule,
            r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE enabled = 1
               ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateScheduleDto) -> sqlx::Result<Schedule> {
        let shell_type = normalize_shell_type(input.shell_type.as_deref())?;
        let schedule_type = normalize_schedule_type(input.schedule_type.as_deref())?;
        let schedule_action = normalize_schedule_action(
            input.schedule_action.as_deref(),
            &schedule_type,
            &input.command,
        )?;
        let enabled = normalize_enabled(input.enabled.unwrap_or(1))?;
        validate_target(
            &schedule_type,
            &schedule_action,
            input.application_id,
            input.compose_id,
            input.server_id,
            input.service_name.as_deref(),
        )?;
        let app_name = input
            .app_name
            .unwrap_or_else(|| generate_schedule_app_name(&input.name));

        sqlx::query_as!(
            Schedule,
            r#"INSERT INTO schedules
               (name, description, cron_expression, app_name, service_name, shell_type,
                schedule_type, schedule_action, command, script, timezone, enabled, application_id, compose_id,
                server_id, organization_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            input.name,
            input.description,
            input.cron_expression,
            app_name,
            input.service_name,
            shell_type,
            schedule_type,
            schedule_action,
            input.command,
            input.script,
            input.timezone,
            enabled,
            input.application_id,
            input.compose_id,
            input.server_id,
            input.organization_id,
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn patch(&self, id: i64, input: PatchScheduleDto) -> sqlx::Result<Schedule> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let cron_expression = input.cron_expression.unwrap_or(current.cron_expression);
        let app_name = input.app_name.unwrap_or(current.app_name);
        let service_name = input.service_name.or(current.service_name);
        let shell_type = match input.shell_type {
            Some(value) => normalize_shell_type(Some(&value))?,
            None => current.shell_type,
        };
        let schedule_type = match input.schedule_type {
            Some(value) => normalize_schedule_type(Some(&value))?,
            None => current.schedule_type,
        };
        let command = input.command.unwrap_or(current.command);
        let schedule_action = match input.schedule_action {
            Some(value) => normalize_schedule_action(Some(&value), &schedule_type, &command)?,
            None => current.schedule_action,
        };
        let script = input.script.or(current.script);
        let timezone = input.timezone.or(current.timezone);
        let enabled = normalize_enabled(input.enabled.unwrap_or(current.enabled))?;
        let application_id = input.application_id.or(current.application_id);
        let compose_id = input.compose_id.or(current.compose_id);
        let server_id = input.server_id.or(current.server_id);
        let organization_id = input.organization_id.or(current.organization_id);
        validate_target(
            &schedule_type,
            &schedule_action,
            application_id,
            compose_id,
            server_id,
            service_name.as_deref(),
        )?;

        sqlx::query_as!(
            Schedule,
            r#"UPDATE schedules SET
               name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?,
               shell_type = ?, schedule_type = ?, schedule_action = ?, command = ?, script = ?, timezone = ?,
               enabled = ?, application_id = ?, compose_id = ?, server_id = ?, organization_id = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            name,
            description,
            cron_expression,
            app_name,
            service_name,
            shell_type,
            schedule_type,
            schedule_action,
            command,
            script,
            timezone,
            enabled,
            application_id,
            compose_id,
            server_id,
            organization_id,
            id,
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn set_enabled(&self, id: i64, enabled: bool) -> sqlx::Result<Schedule> {
        sqlx::query_as!(
            Schedule,
            r#"UPDATE schedules SET enabled = ? WHERE id = ?
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            if enabled { 1 } else { 0 },
            id,
        )
        .fetch_one(self.db.as_ref())
        .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        sqlx::query!("DELETE FROM schedules WHERE id = ?", id)
            .execute(self.db.as_ref())
            .await?;
        Ok(())
    }

    pub async fn run_now(&self, id: i64) -> sqlx::Result<ScheduleRunResult> {
        let schedule = self.get_by_id(id).await?;
        if schedule.enabled == 0 {
            return Err(sqlx::Error::Protocol("schedule is disabled".into()));
        }
        let v = ScheduleType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!(
                "invalid schedule type: {}",
                schedule.shell_type
            ))
        })?;
        match v {
                ScheduleType::Application => self.run_application_schedule(schedule).await,
                ScheduleType::Compose => self.run_compose_schedule(schedule).await,
                ScheduleType::Server | ScheduleType::DokpanelServer => self.run_shell_schedule(schedule).await,
        }
    }

    async fn run_application_schedule(
        &self,
        schedule: Schedule,
    ) -> sqlx::Result<ScheduleRunResult> {
        let application_id = schedule.application_id.ok_or_else(|| {
            sqlx::Error::Protocol("application schedule requires application_id".into())
        })?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_application_container_command(schedule, application_id)
                .await;
        }

        let operation = parse_application_operation(&schedule.schedule_action)?;
        let result = self
            .applications
            .run_operation(application_id, operation)
            .await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "application operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_compose_schedule(&self, schedule: Schedule) -> sqlx::Result<ScheduleRunResult> {
        let compose_id = schedule
            .compose_id
            .ok_or_else(|| sqlx::Error::Protocol("compose schedule requires compose_id".into()))?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_compose_container_command(schedule, compose_id)
                .await;
        }

        let operation = parse_compose_operation(&schedule.schedule_action)?;
        let result = self.compose.run_operation(compose_id, operation).await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "compose operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_shell_schedule(&self, schedule: Schedule) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule
            .script
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(schedule.command.trim());
        if command.is_empty() {
            return Err(sqlx::Error::Protocol("schedule command is empty".into()));
        }

        let executor = if let Some(server_id) = schedule.server_id {
            CommandExecutor::Remote(
                remote_executor(self.db.as_ref(), server_id)
                    .await
                    .map_err(sqlx::Error::Protocol)?,
            )
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!(
                "invalid shell type: {}",
                schedule.shell_type
            ))
        })?;
        let shell = v.executable();
        let output = executor
            .run(shell, ["-lc", command])
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ScheduleRunResult {
            schedule,
            action: "shell".into(),
            deployment_id: None,
            message: "schedule command executed".into(),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
        })
    }

    async fn run_application_container_command(
        &self,
        schedule: Schedule,
        application_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let (app_name, server_id) = sqlx::query_as::<_, (String, Option<i64>)>(
            "SELECT app_name, server_id FROM applications WHERE id = ?",
        )
        .bind(application_id)
        .fetch_one(self.db.as_ref())
        .await?;
        // Applications are always deployed as Swarm services.
        let service_name = format!("{app_name}_{app_name}");
        self.run_swarm_container_command(schedule, server_id, &service_name)
            .await
    }

    async fn run_compose_container_command(
        &self,
        schedule: Schedule,
        compose_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let (app_name, server_id, compose_type) = sqlx::query_as::<_, (String, Option<i64>, String)>(
            "SELECT app_name, server_id, compose_type FROM compose_projects WHERE id = ?",
        )
        .bind(compose_id)
        .fetch_one(self.db.as_ref())
        .await?;

        let compose_service = schedule.service_name.clone().ok_or_else(|| {
            sqlx::Error::Protocol("compose EXEC schedule requires service_name".into())
        })?;
        let v = ComposeType::try_from(compose_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!(
                "invalid compose type: {}",
                compose_type
            ))
        })?;
        match v {
            ComposeType::Stack => {
                let service_name = if compose_service.starts_with(&format!("{app_name}_")) {
                    compose_service
                } else {
                    format!("{app_name}_{compose_service}")
                };
                self.run_swarm_container_command(schedule, server_id, &service_name)
                    .await
            }
            ComposeType::DockerCompose => {

                self.run_compose_project_container_command(
                    schedule, server_id, &app_name, &compose_service,
                )
                .await
            }
        }
    }

    /// Execute command in a Swarm service container (Application or Stack).
    async fn run_swarm_container_command(
        &self,
        schedule: Schedule,
        server_id: Option<i64>,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let filter = format!("label=com.docker.swarm.service.name={service_name}");
        let container = docker
            .containers_raw(false, &[filter.as_str()])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for swarm service {service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!(
                "invalid shell type: {}",
                schedule.shell_type
            ))
        })?;
        let shell = v.executable();
        let output = docker
            .container_exec(&container.id, &[shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on swarm service {service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    /// Execute command in a docker compose container.
    async fn run_compose_project_container_command(
        &self,
        schedule: Schedule,
        server_id: Option<i64>,
        project_name: &str,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let project_filter = format!("label=com.docker.compose.project={project_name}");
        let service_filter = format!("label=com.docker.compose.service={service_name}");
        let container = docker
            .containers_raw(false, &[project_filter.as_str(), service_filter.as_str()])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for compose project={project_name} service={service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!(
                "invalid shell type: {}",
                schedule.shell_type
            ))
        })?;
        let shell = v.executable();
        let output = docker
            .container_exec(&container.id, &[shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on compose {project_name}/{service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    async fn docker_for_server(&self, server_id: Option<i64>) -> sqlx::Result<DockerCli> {
        match server_id {
            Some(server_id) => {
                let executor = remote_executor(self.db.as_ref(), server_id)
                    .await
                    .map_err(sqlx::Error::Protocol)?;
                Ok(DockerCli::from_remote_executor(executor))
            }
            None => Ok(DockerCli::new_local()),
        }
    }
}

async fn select_schedule_by_id(db: &SqlitePool, id: i64) -> sqlx::Result<Schedule> {
    sqlx::query_as!(
        Schedule,
        r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
           shell_type, schedule_type, schedule_action, command, script, timezone, enabled, application_id,
           compose_id, server_id, organization_id, created_at, updated_at
           FROM schedules WHERE id = ?"#,
        id,
    )
    .fetch_one(db)
    .await
}

fn normalize_shell_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("BASH").trim().to_ascii_uppercase();
    let v = ShellType::try_from(value.as_str()).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid shell type: {}", value))
    })?;
    match v {
        ShellType::Bash => Ok(value),
        ShellType::Sh => Ok(value),
    }
}

fn normalize_schedule_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("APPLICATION").trim().to_ascii_uppercase();
    let v = ScheduleType::try_from(value.as_str()).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule type: {}", value))
    })?;
    match v {
        ScheduleType::Application => Ok(value),
        ScheduleType::Compose => Ok(value),
        ScheduleType::Server => Ok(value),
        ScheduleType::DokpanelServer => Ok(value),
    }
}

fn normalize_schedule_action(
    value: Option<&str>,
    schedule_type: &str,
    command: &str,
) -> sqlx::Result<String> {
    let value = value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| infer_schedule_action(schedule_type, command));
    let v = ScheduleAction::try_from(value.as_str()).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule action: {}", value))
    })?;
    match v {
        ScheduleAction::Exec => Ok(value),
        ScheduleAction::Deploy => Ok(value),
        ScheduleAction::Redeploy => Ok(value),
        ScheduleAction::Rebuild => Ok(value),
        ScheduleAction::Reload => Ok(value),
        ScheduleAction::Start => Ok(value),
        ScheduleAction::Stop => Ok(value),
    }
}

fn infer_schedule_action(schedule_type: &str, command: &str) -> String {
    let command = command.trim().to_ascii_lowercase();
    let schedule = ScheduleType::try_from(schedule_type).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type))
    }).unwrap();
    let is_operation = match schedule {
        ScheduleType::Application => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "rebuild" | "reload" | "start"
        ),
        ScheduleType::Compose => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "reload" | "start" | "stop"
        ),
        _ => false,
    };
    if is_operation {
        command.to_ascii_uppercase()
    } else {
        "EXEC".into()
    }
}

fn normalize_enabled(value: i64) -> sqlx::Result<i64> {
    match value {
        0 | 1 => Ok(value),
        _ => Err(sqlx::Error::Protocol("enabled must be 0 or 1".into())),
    }
}

fn validate_target(
    schedule_type: &str,
    schedule_action: &str,
    application_id: Option<i64>,
    compose_id: Option<i64>,
    server_id: Option<i64>,
    service_name: Option<&str>,
) -> sqlx::Result<()> {
    let v = ScheduleType::try_from(schedule_type).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type))
    }).unwrap();
    match v {
        ScheduleType::Application if application_id.is_none() => Err(sqlx::Error::Protocol(
            "APPLICATION schedule requires application_id".into(),
        )),
        ScheduleType::Compose if compose_id.is_none() => Err(sqlx::Error::Protocol(
            "COMPOSE schedule requires compose_id".into(),
        )),
        ScheduleType::Compose
            if schedule_action == "EXEC" && service_name.is_none_or(|v| v.trim().is_empty()) =>
        {
            Err(sqlx::Error::Protocol(
                "COMPOSE EXEC schedule requires service_name".into(),
            ))
        }
        ScheduleType::Server if server_id.is_none() => Err(sqlx::Error::Protocol(
            "SERVER schedule requires server_id".into(),
        )),
        ScheduleType::Server if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "SERVER schedule only supports EXEC action".into(),
        )),
        ScheduleType::DokpanelServer if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "DOKPANEL-SERVER schedule only supports EXEC action".into(),
        )),
        _ => Ok(()),
    }
}

fn parse_application_operation(value: &str) -> sqlx::Result<ApplicationOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str()).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule action: {}", value))
    }).unwrap();
    match  v{
        ScheduleAction::Deploy => Ok(ApplicationOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ApplicationOperation::Redeploy),
        ScheduleAction::Rebuild => Ok(ApplicationOperation::Rebuild),
        ScheduleAction::Reload => Ok(ApplicationOperation::Reload),
        ScheduleAction::Start => Ok(ApplicationOperation::Start),
        _ => Err(sqlx::Error::Protocol(
            "application schedule command must be deploy, redeploy, rebuild, reload or start"
                .into(),
        )),
    }
}

fn parse_compose_operation(value: &str) -> sqlx::Result<ComposeOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str()).map_err(|_| {
        sqlx::Error::Protocol(format!("invalid schedule action: {}", value))
    }).unwrap();
    match v {
        ScheduleAction::Deploy => Ok(ComposeOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ComposeOperation::Redeploy),
        ScheduleAction::Reload => Ok(ComposeOperation::Reload),
        ScheduleAction::Start => Ok(ComposeOperation::Start),
        ScheduleAction::Stop => Ok(ComposeOperation::Stop),
        _ => Err(sqlx::Error::Protocol(
            "compose schedule command must be deploy, redeploy, reload, start or stop".into(),
        )),
    }
}

fn schedule_command(schedule: &Schedule) -> sqlx::Result<&str> {
    let command = schedule
        .script
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(schedule.command.trim());
    if command.is_empty() {
        return Err(sqlx::Error::Protocol("schedule command is empty".into()));
    }
    Ok(command)
}

fn generate_schedule_app_name(name: &str) -> String {
    let slug = name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let slug = if slug.is_empty() {
        "schedule".into()
    } else {
        slug
    };
    format!("{slug}-{}", Uuid::new_v4().simple())
}
