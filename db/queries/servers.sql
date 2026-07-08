-- ============================================================
-- SERVERS
-- ============================================================

-- name: GetServerByID :one
SELECT * FROM servers WHERE id = ?;

-- name: GetServerByAppName :one
SELECT * FROM servers WHERE app_name = ?;

-- name: ListServers :many
SELECT * FROM servers ORDER BY created_at DESC;

-- name: ListServersByStatus :many
SELECT * FROM servers WHERE server_status = ? ORDER BY created_at DESC;

-- name: ListServersByType :many
SELECT * FROM servers WHERE server_type = ? ORDER BY created_at DESC;

-- name: CreateServer :one
INSERT INTO servers (
    name, description, ip_address, port, username,
    app_name, server_status, server_type,
    enable_docker_cleanup, log_cleanup_cron,
    command, metrics_config, ssh_key_id
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateServer :one
UPDATE servers
SET
    name              = ?,
    description       = ?,
    ip_address        = ?,
    port              = ?,
    username          = ?,
    server_status     = ?,
    server_type       = ?,
    enable_docker_cleanup = ?,
    log_cleanup_cron  = ?,
    command           = ?,
    metrics_config    = ?,
    ssh_key_id        = ?
WHERE id = ?
RETURNING *;

-- name: UpdateServerStatus :exec
UPDATE servers SET server_status = ? WHERE id = ?;

-- name: UpdateServerMetricsConfig :exec
UPDATE servers SET metrics_config = ? WHERE id = ?;

-- name: DeleteServer :exec
DELETE FROM servers WHERE id = ?;
