-- Migration: 0006_rbac_schema.sql
-- Add RBAC tables (user_policy, resource_access), system group flag, and owner index

-- Add is_system flag to groups table
ALTER TABLE groups ADD COLUMN is_system INTEGER NOT NULL DEFAULT 0;

-- Add is_owner flag to users table
ALTER TABLE users ADD COLUMN is_owner INTEGER DEFAULT 0;

-- Unique index for single owner
CREATE UNIQUE INDEX IF NOT EXISTS idx_single_owner ON users(is_owner) WHERE is_owner = 1;

-- Create user_policy table for explicit user permission overrides
CREATE TABLE IF NOT EXISTS user_policy (
	id         INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	org_id     INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	policy_id  INTEGER NOT NULL REFERENCES policy(id) ON DELETE CASCADE,
	effect     TEXT NOT NULL DEFAULT 'GRANT',
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT effect_check CHECK (effect IN ('GRANT', 'DENY')),
	UNIQUE(user_id, org_id, policy_id)
) STRICT;

-- Create resource_access table for granular resource permissions
CREATE TABLE IF NOT EXISTS resource_access (
	id            INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id       INTEGER REFERENCES users(id) ON DELETE CASCADE,
	org_id        INTEGER REFERENCES organization(id) ON DELETE CASCADE,
	resource_type TEXT,
	resource_id   INTEGER,
	created_at    INTEGER DEFAULT (strftime('%s', 'now')),
	CONSTRAINT resource_type_check CHECK (
		resource_type IN ('PROJECT', 'SERVER', 'ENVIRONMENT', 'SERVICE', 'GIT_PROVIDER')
	)
) STRICT;
