-- Add migration script here
-- Create Users Table
create table users (
  id serial primary key,
  email text not null unique,
  password text not null,
  username text not null,
  password_changed_at timestamptz not null default('0001-01-01 00:00:00Z'),
  created_at timestamptz not null default (now())
);
