-- Add migration script here
create table user_avatars
(
    id         INTEGER
        primary key autoincrement,
    href       TEXT not null,
    path       TEXT not null,
    hash       TEXT NOT NULL,
    created_at DATETIME default CURRENT_TIMESTAMP,
    updated_at DATETIME default CURRENT_TIMESTAMP,
    deleted_at DATETIME
);
