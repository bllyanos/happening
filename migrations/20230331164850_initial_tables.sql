-- Add migration script here
CREATE TABLE users (
  id bigserial primary key,
  username varchar(64) unique not null,
  email varchar(255) unique not null,
  name varchar(255) not null,
  password varchar(64) not null,
  is_verified boolean not null default false,
  created_at timestamptz not null default NOW(),
  verified_at timestamptz
);