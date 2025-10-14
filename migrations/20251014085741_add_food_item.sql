-- Add migration script here
create table if not exists food_items(
    id uuid primary key,
    name varchar(255) not null,
    description varchar(255),
    price numeric(10, 2) not null,
    discount numeric(10, 2),
    restaurant_id uuid not null references restaurants(id),
    cuisine_id uuid references cuisines(id),
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);