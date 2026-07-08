-- Web Server Settings Table
CREATE TABLE settings (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	server_ip TEXT,
	-- certificate_type: NONE | LETSENCRYPT | CUSTOM
	certificate_type TEXT NOT NULL DEFAULT 'NONE',
	custom_cert_resolver TEXT,
	https INTEGER NOT NULL DEFAULT 0,
	host TEXT, -- Domain Name for server
	lets_encrypt_email TEXT,
	enable_docker_cleanup INTEGER NOT NULL DEFAULT 1,
	log_cleanup_cron TEXT DEFAULT '0 0 * * *',
	-- JSON: metrics config object { server: {...}, containers: {...} }
	metrics_config TEXT NOT NULL DEFAULT '',
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT settings_certificate_check CHECK (certificate_type IN ('NONE', 'LETSENCRYPT', 'CUSTOM'))
) STRICT;

-- Ai Settings Table
CREATE TABLE ai_settings (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	api_url TEXT NOT NULL,
	api_key TEXT NOT NULL,
	model TEXT NOT NULL,
	is_enabled INTEGER NOT NULL DEFAULT 1,
	-- Foreign keys
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT ai_enabled_check CHECK (is_enabled IN (0, 1))
) STRICT;

-- Indexes for faster queries
CREATE INDEX IF NOT EXISTS idx_ai_settings_organization_id ON ai_settings(organization_id);

-- Trigger Function
CREATE TRIGGER settings_updated_at
AFTER UPDATE ON settings
FOR EACH ROW
BEGIN
	UPDATE settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER ai_settings_updated_at
AFTER UPDATE ON ai_settings
FOR EACH ROW
BEGIN
	UPDATE ai_settings
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;