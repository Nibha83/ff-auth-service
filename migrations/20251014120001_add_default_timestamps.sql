-- Add default values for created_at and updated_at columns
-- This allows these columns to be automatically populated when not explicitly provided

-- Add default values to access_tokens table
ALTER TABLE access_tokens 
ALTER COLUMN created_at SET DEFAULT now();

ALTER TABLE access_tokens 
ALTER COLUMN updated_at SET DEFAULT now();

-- Add default values to user_roles table
ALTER TABLE user_roles 
ALTER COLUMN created_at SET DEFAULT now();

ALTER TABLE user_roles 
ALTER COLUMN updated_at SET DEFAULT now();
