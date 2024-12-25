-- Add migration script here
create table environment_proxies
(
    id         INTEGER
        primary key autoincrement,
    kind       TEXT not null,
    host       TEXT not null,
    port       TEXT not null,
    username   TEXT,
    password   TEXT,
    user_uuid  INTEGER,
    environment_group_id  INTEGER,
    created_at DATETIME default CURRENT_TIMESTAMP,
    updated_at DATETIME default CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    foreign key (user_uuid) references users (uuid),
    foreign key (environment_group_id) references  environment_proxy_groups (uuid)
);