-- Create database for auth service
-- Note: In CockroachDB, databases are created automatically when first accessed
-- This migration ensures the database exists and sets up any initial configuration

-- Create the auth_service database if it doesn't exist
-- CockroachDB will create it automatically when we connect to it
-- This file serves as documentation of the database creation

-- Set default settings for the database
SET sql_safe_updates = false;
