-- Health checks report table
CREATE TABLE IF NOT EXISTS health_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    target_id INTEGER NOT NULL,
    target_type TEXT NOT NULL, -- 'APPLICATION', 'DATABASE', 'SERVER'
    status TEXT NOT NULL,      -- 'HEALTHY', 'UNHEALTHY', 'DEGRADED'
    response_time_ms INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX IF NOT EXISTS idx_health_reports_target ON health_reports(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_health_reports_created ON health_reports(created_at);

-- Alerting rules configuration
CREATE TABLE IF NOT EXISTS alert_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    target_type TEXT NOT NULL, -- 'SERVER', 'APPLICATION', 'DATABASE'
    target_id INTEGER NOT NULL,
    metric_name TEXT NOT NULL, -- 'CPU', 'MEMORY', 'DISK', 'HEALTH'
    operator TEXT NOT NULL,    -- 'GT', 'GTE', 'LT', 'LTE', 'EQ'
    threshold REAL NOT NULL,
    duration_seconds INTEGER NOT NULL DEFAULT 60,
    notification_channel TEXT NOT NULL DEFAULT 'SYSTEM',
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX IF NOT EXISTS idx_alert_rules_target ON alert_rules(target_type, target_id);
