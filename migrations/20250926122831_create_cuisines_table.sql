-- Add migration script here
create table cuisines(
    id uuid not null primary key,
    restaurant_id uuid not null references restaurants(id),
    name STRING NOT NULL,
    description STRING,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);