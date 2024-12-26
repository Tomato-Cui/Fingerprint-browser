-- Add migration script here
create table user_infos
(
    id          INTEGER
        primary key autoincrement,
    nickname    TEXT not null unique,
    password    TEXT not null,
    description TEXT,
    email       TEXT not null UNIQUE,
    phone       TEXT,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);
