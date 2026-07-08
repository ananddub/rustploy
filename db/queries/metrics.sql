-- name: SaveServerMetric :exec
INSERT INTO server_metrics (
	timestamp, cpu, cpu_model, cpu_cores,
	cpu_physical_cores, cpu_speed, os,
	distro, kernel, arch, mem_used, mem_used_gb,
	mem_total, uptime, disk_used, total_disk,
	network_in, network_out
)
VALUES (
	?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
	?, ?
);

-- name: GetLastNServerMetrics :many
WITH recent_metrics AS (
	SELECT *
	FROM server_metrics
	ORDER BY timestamp DESC
	LIMIT ?
)
SELECT * FROM recent_metrics ORDER BY timestamp ASC;

-- name: DeleteOldServerMetrics :exec
DELETE FROM server_metrics
WHERE timestamp < CAST(strftime('%s', 'now', ?) AS INTEGER);

-- name: SaveContainerMetric :exec
INSERT INTO container_metrics (
	timestamp, container_id, container_name, metrics_json
) VALUES (
	?, ?, ?, ?
);

-- name: GetLastNContainerMetrics :many
WITH recent_container_metrics AS (
	SELECT *
	FROM container_metrics
	WHERE container_name = ? OR container_name LIKE ?
	ORDER BY timestamp DESC
	LIMIT ?
)
SELECT * FROM recent_container_metrics ORDER BY timestamp ASC;

-- name: DeleteOldContainerMetrics :exec
DELETE FROM container_metrics
WHERE timestamp < CAST(strftime('%s', 'now', ?) AS INTEGER);