-- Add migration script here
create table teams
(
    id          INTEGER
        primary key autoincrement,
    name        TEXT,
    description INTEGER,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);