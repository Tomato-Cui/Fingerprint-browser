-- Add migration script here
create table team_group_permission
(
    id          INTEGER
        primary key autoincrement,
    name        TEXT,
    description INTEGER,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);
