-- ============================================================
-- DESTINATIONS (Storage)
-- ============================================================

-- name: GetDestinationByID :one
SELECT * FROM destinations WHERE id = ?;

-- name: ListDestinationsByOrg :many
SELECT * FROM destinations WHERE organization_id = ? ORDER BY created_at DESC;

-- name: CreateDestination :one
INSERT INTO destinations (
    name, provider, access_key, secret_access_key,
    bucket, region, endpoint, additional_flags, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateDestination :one
UPDATE destinations
SET
    name              = ?,
    provider          = ?,
    access_key        = ?,
    secret_access_key = ?,
    bucket            = ?,
    region            = ?,
    endpoint          = ?,
    additional_flags  = ?
WHERE id = ?
RETURNING *;

-- name: DeleteDestination :exec
DELETE FROM destinations WHERE id = ?;

-- ============================================================
-- BACKUPS (Database Backup Jobs)
-- ============================================================

-- name: GetBackupByID :one
SELECT * FROM backups WHERE id = ?;

-- name: GetBackupByAppName :one
SELECT * FROM backups WHERE app_name = ?;

-- name: ListBackupsByOrg :many
SELECT * FROM backups WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListBackupsByDestination :many
SELECT * FROM backups WHERE destination_id = ? ORDER BY created_at DESC;

-- name: ListBackupsByType :many
SELECT * FROM backups WHERE backup_type = ? AND organization_id = ? ORDER BY created_at DESC;

-- name: CreateBackup :one
INSERT INTO backups (
    app_name, schedule, enabled, database_name, prefix,
    service_name, keep_latest_count, backup_type, database_type, metadata,
    compose_id, postgres_id, mysql_id, mariadb_id,
    mongo_id, redis_id, libsql_id,
    destination_id, organization_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateBackup :one
UPDATE backups
SET
    schedule          = ?,
    enabled           = ?,
    database_name     = ?,
    prefix            = ?,
    service_name      = ?,
    keep_latest_count = ?,
    metadata          = ?
WHERE id = ?
RETURNING *;

-- name: ToggleBackup :exec
UPDATE backups SET enabled = ? WHERE id = ?;

-- name: DeleteBackup :exec
DELETE FROM backups WHERE id = ?;

-- ============================================================
-- VOLUME BACKUPS
-- ============================================================

-- name: GetVolumeBackupByID :one
SELECT * FROM volume_backups WHERE id = ?;

-- name: GetVolumeBackupByAppName :one
SELECT * FROM volume_backups WHERE app_name = ?;

-- name: ListVolumeBackupsByOrg :many
SELECT * FROM volume_backups WHERE organization_id = ? ORDER BY created_at DESC;

-- name: ListVolumeBackupsByApplication :many
SELECT * FROM volume_backups WHERE application_id = ? ORDER BY created_at DESC;

-- name: CreateVolumeBackup :one
INSERT INTO volume_backups (
    name, volume_name, prefix, service_type, app_name,
    service_name, turn_off, cron_expression, keep_latest_count, enabled,
    destination_id, organization_id,
    application_id, postgres_id, mysql_id, mariadb_id,
    mongo_id, redis_id, libsql_id, compose_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateVolumeBackup :one
UPDATE volume_backups
SET
    name              = ?,
    volume_name       = ?,
    prefix            = ?,
    service_name      = ?,
    turn_off          = ?,
    cron_expression   = ?,
    keep_latest_count = ?,
    enabled           = ?
WHERE id = ?
RETURNING *;

-- name: ToggleVolumeBackup :exec
UPDATE volume_backups SET enabled = ? WHERE id = ?;

-- name: DeleteVolumeBackup :exec
DELETE FROM volume_backups WHERE id = ?;
