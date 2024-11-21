-- Add migration script here
alter table users
add column is_bot boolean not null default false;
