-- Add migration script here
create table proxies
(
    id         INTEGER
        primary key autoincrement,
    kind       TEXT not null,
    host       TEXT not null,
    port       TEXT not null,
    username   TEXT not null,
    password   TEXT not null,
    user_uuid  INTEGER,
    created_at DATETIME default CURRENT_TIMESTAMP,
    updated_at DATETIME default CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    foreign key (user_uuid) references users (uuid)
);