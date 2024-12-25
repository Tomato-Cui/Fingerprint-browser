-- Add migration script here
create table environment_accounts
(
    id                   INTEGER
        primary key autoincrement,
    platform             TEXT not null,
    platform_url         TEXT not null,
    platform_description TEXT,
    environment_uuid     INTEGER,
    user_uuid            INTEGER,
    created_at           DATETIME default CURRENT_TIMESTAMP,
    updated_at           DATETIME default CURRENT_TIMESTAMP,
    deleted_at           DATETIME,
    foreign key (environment_uuid) references environments (uuid),
    foreign key (user_uuid) references users (uuid)

);