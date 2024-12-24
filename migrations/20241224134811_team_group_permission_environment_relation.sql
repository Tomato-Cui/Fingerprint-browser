-- Add migration script here
create table team_group_permission_environment_relation
(
    id          INTEGER
        primary key autoincrement,
    environment_uuid        integer,
    team_group_permission_id INTEGER,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);
