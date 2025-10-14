-- Add migration script here
ALTER TABLE user_roles
ADD CONSTRAINT if not exists user_roles_user_role_key UNIQUE (user_role);
