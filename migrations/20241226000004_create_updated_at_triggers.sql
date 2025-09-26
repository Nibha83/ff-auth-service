-- Note: CockroachDB doesn't support traditional PostgreSQL triggers
-- Instead, we'll use application-level timestamp updates
-- This file serves as documentation for the intended behavior

-- In CockroachDB, we handle updated_at timestamps at the application level
-- The columns are already created with DEFAULT NOW() in the table creation

-- Alternative approach for CockroachDB would be to use:
-- 1. Application-level updates (recommended)
-- 2. Computed columns (limited functionality)
-- 3. Stored procedures called explicitly

-- For now, we'll rely on the application to update the updated_at field
-- when performing UPDATE operations

-- This is a placeholder migration to maintain migration sequence
SELECT 1;
