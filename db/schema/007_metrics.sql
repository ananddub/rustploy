-- Table to store host server metrics
CREATE TABLE server_metrics (
	timestamp INTEGER PRIMARY KEY,
	cpu REAL NOT NULL,
	cpu_model TEXT NOT NULL,
	cpu_cores INTEGER NOT NULL,
	cpu_physical_cores INTEGER NOT NULL,
	cpu_speed REAL NOT NULL,
	os TEXT NOT NULL,
	distro TEXT NOT NULL,
	kernel TEXT NOT NULL,
	arch TEXT NOT NULL,
	mem_used REAL NOT NULL,
	mem_used_gb REAL NOT NULL,
	mem_total REAL NOT NULL,
	uptime INTEGER NOT NULL,
	disk_used REAL NOT NULL,
	total_disk REAL NOT NULL,
	network_in REAL NOT NULL,
	network_out REAL NOT NULL
) STRICT;

-- Table to store individual docker container metrics
CREATE TABLE container_metrics (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	timestamp INTEGER NOT NULL,
	container_id TEXT NOT NULL,
	container_name TEXT NOT NULL,
	metrics_json TEXT NOT NULL
) STRICT;

-- Indexes for improve query performance
CREATE INDEX idx_container_metrics_timestamp ON container_metrics(timestamp);
CREATE INDEX idx_container_metrics_name ON container_metrics(container_name);