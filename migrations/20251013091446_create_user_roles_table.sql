-- Add migration script here
create table if not exists user_roles(
    id uuid primary key,
    user_role varchar(255) not null,
    created_at timestamp not null,
    updated_at timestamp not null
);