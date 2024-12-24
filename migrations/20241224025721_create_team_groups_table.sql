-- Add migration script here

create table
    team_groups
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT,
    description TEXT,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);