-- Add migration script here
INSERT INTO user_roles (id, user_role, created_at, updated_at)
VALUES (gen_random_uuid(), 'admin', now(), now());
