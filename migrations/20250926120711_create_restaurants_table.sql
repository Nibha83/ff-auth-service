-- Add migration script here
create table restaurants (
    id uuid not null primary key,
    name STRING NOT NULL,
    email STRING NOT NULL UNIQUE,
    password STRING NOT NULL,
    contact_number varchar(20) not null,
    address varchar(255) not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);

