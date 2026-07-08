CREATE TABLE users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	email TEXT UNIQUE,
	last_name TEXT,
	first_name TEXT,
	avatar TEXT NOT NULL,
	-- User role: OWNER | ADMIN | MEMBER
	role TEXT DEFAULT 'OWNER',
	about_me TEXT,
	password TEXT NOT NULL,
	is_email_verify INTEGER DEFAULT 0,
	email_verify_at INTEGER,
	two_factor_enable INTEGER DEFAULT 0,
	is_registered INTEGER DEFAULT 0 NOT NULL,
	added_by INTEGER DEFAULT NULL REFERENCES users(id),
	group_id INTEGER NOT NULL REFERENCES groups(id),
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('OWNER', 'ADMIN', 'MEMBER'))
) STRICT;

CREATE TABLE two_factor (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	secret TEXT NOT NULL,
	backup_codes TEXT NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
) STRICT;

CREATE TABLE jwt_tokens (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	jti TEXT NOT NULL,
	-- Role at time of token issuance: OWNER | ADMIN | MEMBER
	role TEXT NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	is_blacklist INTEGER DEFAULT 0,
	blacklist_at INTEGER,
	expired_at INTEGER,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('OWNER', 'ADMIN', 'MEMBER'))
) STRICT;

-- Trigger Function
CREATE TRIGGER users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
	UPDATE users
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER jwt_tokens_updated_at
AFTER UPDATE ON jwt_tokens
FOR EACH ROW
BEGIN
	UPDATE jwt_tokens
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;