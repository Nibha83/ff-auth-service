-- Add migration script here
create table access_tokens (
    id uuid primary key,
    access_token varchar(255) not null,
    user_role varchar(255) not null references user_roles(user_role),
    user_id uuid not null,
    created_at timestamp not null,
    updated_at timestamp not null
);