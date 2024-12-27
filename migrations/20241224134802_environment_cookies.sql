-- Add migration script here
create table environment_cookies
(
    id               INTEGER
        primary key autoincrement,
    value            TEXT not null,
    environment_uuid TEXT,
    created_at       DATETIME default CURRENT_TIMESTAMP,
    updated_at       DATETIME default CURRENT_TIMESTAMP,
    deleted_at       DATETIME,
    foreign key (environment_uuid) references environments (uuid)
);
