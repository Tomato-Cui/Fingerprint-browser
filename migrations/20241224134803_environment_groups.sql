-- Add migration script here

create table environment_groups
(
    id          INTEGER
        primary key autoincrement,
    user_uuid   TEXT not null,
    name        TEXT not null,
    description TEXT,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME,
    foreign key (user_uuid) references users (uuid)
);