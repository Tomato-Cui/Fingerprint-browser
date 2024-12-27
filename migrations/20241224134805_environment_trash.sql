-- Add migration script here
create table environment_trash
(
    id               INTEGER
        primary key autoincrement,
    environment_uuid TEXT not null,
    from_user_uuid       TEXT,
    created_at       DATETIME default CURRENT_TIMESTAMP,
    updated_at       DATETIME default CURRENT_TIMESTAMP,
    deleted_at       DATETIME,
    foreign key (environment_uuid) references environments (uuid),
    foreign key (from_user_uuid) references users (uuid)
);