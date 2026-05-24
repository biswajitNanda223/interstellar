CREATE SCHEMA IF NOT EXISTS careerforge;
CREATE SCHEMA IF NOT EXISTS task_manager;

CREATE TABLE IF NOT EXISTS careerforge.schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS task_manager.schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

