-- ============================================================
-- SCHEDULES
-- ============================================================

-- name: GetScheduleByID :one
SELECT * FROM schedules WHERE id = ?;

-- name: GetScheduleByAppName :one
SELECT * FROM schedules WHERE app_name = ?;

-- name: ListSchedulesByOrg :many
SELECT * FROM schedules WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByApplication :many
SELECT * FROM schedules WHERE application_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByCompose :many
SELECT * FROM schedules WHERE compose_id = ? ORDER BY created_at DESC;

-- name: ListSchedulesByServer :many
SELECT * FROM schedules WHERE server_id = ? ORDER BY created_at DESC;

-- name: ListEnabledSchedules :many
SELECT * FROM schedules WHERE enabled = 1 ORDER BY created_at DESC;

-- name: CreateSchedule :one
INSERT INTO schedules (
    name, description, cron_expression, app_name,
    service_name, shell_type, schedule_type, schedule_action, command, script,
    timezone, enabled,
    application_id, compose_id, server_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateSchedule :one
UPDATE schedules
SET
    name            = ?,
    description     = ?,
    cron_expression = ?,
    service_name    = ?,
    shell_type      = ?,
    schedule_action = ?,
    command         = ?,
    script          = ?,
    timezone        = ?
WHERE id = ?
RETURNING *;

-- name: ToggleSchedule :exec
UPDATE schedules SET enabled = ? WHERE id = ?;

-- name: DeleteSchedule :exec
DELETE FROM schedules WHERE id = ?;
