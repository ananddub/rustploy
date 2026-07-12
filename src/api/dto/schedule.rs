use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{db::models::schedules::Schedule, services::schedule::ScheduleRunResult};

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateScheduleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub cron_expression: String,
    #[validate(length(min = 1, max = 255))]
    pub app_name: Option<String>,
    #[validate(length(max = 255))]
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    #[validate(length(min = 1))]
    pub command: String,
    pub script: Option<String>,
    pub timezone: Option<String>,
    #[serde(default)]
    pub enabled: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct PatchScheduleDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub cron_expression: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub app_name: Option<String>,
    #[validate(length(max = 255))]
    pub service_name: Option<String>,
    pub shell_type: Option<String>,
    pub schedule_type: Option<String>,
    #[validate(length(min = 1))]
    pub command: Option<String>,
    pub script: Option<String>,
    pub timezone: Option<String>,
    pub enabled: Option<i64>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ScheduleResponseDto {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub cron_expression: String,
    pub app_name: String,
    pub service_name: Option<String>,
    pub shell_type: String,
    pub schedule_type: String,
    pub command: String,
    pub script: Option<String>,
    pub timezone: Option<String>,
    pub enabled: i64,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub server_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Schedule> for ScheduleResponseDto {
    fn from(value: Schedule) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            cron_expression: value.cron_expression,
            app_name: value.app_name,
            service_name: value.service_name,
            shell_type: value.shell_type,
            schedule_type: value.schedule_type,
            command: value.command,
            script: value.script,
            timezone: value.timezone,
            enabled: value.enabled,
            application_id: value.application_id,
            compose_id: value.compose_id,
            server_id: value.server_id,
            organization_id: value.organization_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ScheduleRunResponseDto {
    pub schedule: ScheduleResponseDto,
    pub action: String,
    pub deployment_id: Option<i64>,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl From<ScheduleRunResult> for ScheduleRunResponseDto {
    fn from(value: ScheduleRunResult) -> Self {
        Self {
            schedule: ScheduleResponseDto::from(value.schedule),
            action: value.action,
            deployment_id: value.deployment_id,
            message: value.message,
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}
