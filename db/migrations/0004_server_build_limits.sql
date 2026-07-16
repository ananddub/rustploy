-- Add build limit columns to servers table
ALTER TABLE servers ADD COLUMN build_memory_limit TEXT;
ALTER TABLE servers ADD COLUMN build_cpu_limit TEXT;
