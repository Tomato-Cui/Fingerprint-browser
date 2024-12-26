-- Add migration script here
create table resource_whitelist
(
    id          INTEGER
        primary key autoincrement,
    path        TEXT not null,
    method      TEXT not null,
    description TEXT,
    can_use     INTEGER,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);


