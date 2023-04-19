-- Add migration script here
CREATE TABLE workspaces (
  id bigserial primary key,
  owner_id bigint not null,
  name varchar(255) not null,
  slug varchar(255) unique not null,
  created_at timestamptz not null default NOW(),
  constraint fx_owner_id
    foreign key(owner_id)
      references users(id)
);

CREATE INDEX idx_workspaces_name ON workspaces(name);
CREATE INDEX idx_workspaces_owner_id ON workspaces(owner_id);