-- Add migration script here
drop table deliverty_partners;

create table delivery_partners (
    id uuid not null primary key,
    name STRING NOT NULL,
    email STRING NOT NULL UNIQUE,
    password STRING NOT NULL,
    phone_number varchar(20) not null,
    vehicle_number varchar(20) not null,
    rating float not null default 0,
    is_available boolean not null default false,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now()
);
