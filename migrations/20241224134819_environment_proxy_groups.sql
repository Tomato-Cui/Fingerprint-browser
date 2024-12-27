-- Add migration script here
create table environment_proxy_groups
(
    id         INTEGER primary key autoincrement,
    name        TEXT not null,
    description TEXT,
    user_uuid   TEXT,
    created_at DATETIME default CURRENT_TIMESTAMP,
    updated_at DATETIME default CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    foreign key (user_uuid) references users (uuid)
);