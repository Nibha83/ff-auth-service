-- Rollback script to drop all created objects
-- Use this file if you need to completely reset the database

-- Note: This migration is for documentation purposes
-- CockroachDB doesn't support IF EXISTS for triggers/functions that don't exist
-- Manual cleanup would involve:

-- 1. Drop tables (addresses first due to foreign key constraint)
-- DROP TABLE addresses;
-- DROP TABLE customers;

-- 2. Drop database (from a different database connection)
-- DROP DATABASE auth_service;

-- For now, this is a no-op migration to maintain sequence
SELECT 1;
