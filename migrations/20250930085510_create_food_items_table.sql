-- Add migration script here
create table food_items(
    id uuid not null primary key,
    name STRING NOT NULL,
    price float,
    restaurant_id uuid not null references restaurants(id),
    cuisine_id uuid references cuisines(id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);