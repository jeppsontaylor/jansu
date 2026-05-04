-- Phase 10 / Phase 08 PostgreSQL migration
-- Applies schema changes required for consumer group offsets and leader epoch tracking
-- against an existing Jansu PostgreSQL database that was initialized before these features.
--
-- This migration is IDEMPOTENT — safe to run multiple times.
--
-- Changes:
--   1. leader_epoch_history table (Phase 08)
--   2. expires_at column on consumer_offset (Phase 10)
--
-- Usage:
--   psql -h localhost -U postgres -d jansu -f pg_migration_phase10.sql

BEGIN;

-- Phase 08: Leader epoch history for partition leader tracking
CREATE TABLE IF NOT EXISTS leader_epoch_history (
    topition int REFERENCES topition (id) ON DELETE CASCADE,
    epoch int NOT NULL,
    start_offset bigint NOT NULL,
    last_updated timestamp DEFAULT current_timestamp NOT NULL,
    created_at timestamp DEFAULT current_timestamp NOT NULL,
    PRIMARY KEY (topition, epoch)
);

-- Phase 10: Expiry timestamp for consumer offset retention
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'consumer_offset' AND column_name = 'expires_at'
    ) THEN
        ALTER TABLE consumer_offset ADD COLUMN expires_at timestamp;
    END IF;
END $$;

COMMIT;
