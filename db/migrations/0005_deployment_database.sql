-- Add database_id and database_kind columns to deployments table
ALTER TABLE deployments ADD COLUMN database_id INTEGER;
ALTER TABLE deployments ADD COLUMN database_kind TEXT;
