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
    utils::exec::{CommandExecutor, LocalExecutor},
};

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
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
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
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
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
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
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
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at
               FROM schedules WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC"#,
            organization_id,
        )
        .fetch_all(self.db.as_ref())
        .await
    }

    pub async fn create(&self, input: CreateScheduleDto) -> sqlx::Result<Schedule> {
        let shell_type = normalize_shell_type(input.shell_type.as_deref())?;
        let schedule_type = normalize_schedule_type(input.schedule_type.as_deref())?;
        let enabled = normalize_enabled(input.enabled.unwrap_or(1))?;
        validate_target(
            &schedule_type,
            input.application_id,
            input.compose_id,
            input.server_id,
        )?;
        let app_name = input
            .app_name
            .unwrap_or_else(|| generate_schedule_app_name(&input.name));

        sqlx::query_as!(
            Schedule,
            r#"INSERT INTO schedules
               (name, description, cron_expression, app_name, service_name, shell_type,
                schedule_type, command, script, timezone, enabled, application_id, compose_id,
                server_id, organization_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            input.name,
            input.description,
            input.cron_expression,
            app_name,
            input.service_name,
            shell_type,
            schedule_type,
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
        let script = input.script.or(current.script);
        let timezone = input.timezone.or(current.timezone);
        let enabled = normalize_enabled(input.enabled.unwrap_or(current.enabled))?;
        let application_id = input.application_id.or(current.application_id);
        let compose_id = input.compose_id.or(current.compose_id);
        let server_id = input.server_id.or(current.server_id);
        let organization_id = input.organization_id.or(current.organization_id);
        validate_target(&schedule_type, application_id, compose_id, server_id)?;

        sqlx::query_as!(
            Schedule,
            r#"UPDATE schedules SET
               name = ?, description = ?, cron_expression = ?, app_name = ?, service_name = ?,
               shell_type = ?, schedule_type = ?, command = ?, script = ?, timezone = ?,
               enabled = ?, application_id = ?, compose_id = ?, server_id = ?, organization_id = ?
               WHERE id = ?
               RETURNING id AS "id?", name, description, cron_expression, app_name, service_name,
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
               compose_id, server_id, organization_id, created_at, updated_at"#,
            name,
            description,
            cron_expression,
            app_name,
            service_name,
            shell_type,
            schedule_type,
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
               shell_type, schedule_type, command, script, timezone, enabled, application_id,
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

        match schedule.schedule_type.as_str() {
            "APPLICATION" => self.run_application_schedule(schedule).await,
            "COMPOSE" => self.run_compose_schedule(schedule).await,
            "SERVER" | "DOKPANEL-SERVER" => self.run_shell_schedule(schedule).await,
            other => Err(sqlx::Error::Protocol(format!(
                "unsupported schedule type: {other}"
            ))),
        }
    }

    async fn run_application_schedule(
        &self,
        schedule: Schedule,
    ) -> sqlx::Result<ScheduleRunResult> {
        let application_id = schedule.application_id.ok_or_else(|| {
            sqlx::Error::Protocol("application schedule requires application_id".into())
        })?;
        let operation = parse_application_operation(&schedule.command)?;
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
        let operation = parse_compose_operation(&schedule.command)?;
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
        let shell = match schedule.shell_type.as_str() {
            "SH" => "sh",
            _ => "bash",
        };
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
}

async fn select_schedule_by_id(db: &SqlitePool, id: i64) -> sqlx::Result<Schedule> {
    sqlx::query_as!(
        Schedule,
        r#"SELECT id AS "id?", name, description, cron_expression, app_name, service_name,
           shell_type, schedule_type, command, script, timezone, enabled, application_id,
           compose_id, server_id, organization_id, created_at, updated_at
           FROM schedules WHERE id = ?"#,
        id,
    )
    .fetch_one(db)
    .await
}

fn normalize_shell_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("BASH").trim().to_ascii_uppercase();
    match value.as_str() {
        "BASH" | "SH" => Ok(value),
        _ => Err(sqlx::Error::Protocol(
            "shell_type must be BASH or SH".into(),
        )),
    }
}

fn normalize_schedule_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("APPLICATION").trim().to_ascii_uppercase();
    match value.as_str() {
        "APPLICATION" | "COMPOSE" | "SERVER" | "DOKPANEL-SERVER" => Ok(value),
        _ => Err(sqlx::Error::Protocol(
            "schedule_type must be APPLICATION, COMPOSE, SERVER or DOKPANEL-SERVER".into(),
        )),
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
    application_id: Option<i64>,
    compose_id: Option<i64>,
    server_id: Option<i64>,
) -> sqlx::Result<()> {
    match schedule_type {
        "APPLICATION" if application_id.is_none() => Err(sqlx::Error::Protocol(
            "APPLICATION schedule requires application_id".into(),
        )),
        "COMPOSE" if compose_id.is_none() => Err(sqlx::Error::Protocol(
            "COMPOSE schedule requires compose_id".into(),
        )),
        "SERVER" if server_id.is_none() => Err(sqlx::Error::Protocol(
            "SERVER schedule requires server_id".into(),
        )),
        _ => Ok(()),
    }
}

fn parse_application_operation(value: &str) -> sqlx::Result<ApplicationOperation> {
    match value.trim().to_ascii_lowercase().as_str() {
        "deploy" => Ok(ApplicationOperation::Deploy),
        "redeploy" => Ok(ApplicationOperation::Redeploy),
        "rebuild" => Ok(ApplicationOperation::Rebuild),
        "reload" => Ok(ApplicationOperation::Reload),
        "start" => Ok(ApplicationOperation::Start),
        _ => Err(sqlx::Error::Protocol(
            "application schedule command must be deploy, redeploy, rebuild, reload or start"
                .into(),
        )),
    }
}

fn parse_compose_operation(value: &str) -> sqlx::Result<ComposeOperation> {
    match value.trim().to_ascii_lowercase().as_str() {
        "deploy" => Ok(ComposeOperation::Deploy),
        "redeploy" => Ok(ComposeOperation::Redeploy),
        "reload" => Ok(ComposeOperation::Reload),
        "start" => Ok(ComposeOperation::Start),
        "stop" => Ok(ComposeOperation::Stop),
        _ => Err(sqlx::Error::Protocol(
            "compose schedule command must be deploy, redeploy, reload, start or stop".into(),
        )),
    }
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
