-- Add migration script here
INSERT INTO user_roles (id, user_role, created_at, updated_at)
VALUES (gen_random_uuid(), 'restaurant', now(), now());
INSERT INTO user_roles (id, user_role, created_at, updated_at)
VALUES (gen_random_uuid(), 'customer', now(), now());
INSERT INTO user_roles (id, user_role, created_at, updated_at)
VALUES (gen_random_uuid(), 'delivery_partner', now(), now());