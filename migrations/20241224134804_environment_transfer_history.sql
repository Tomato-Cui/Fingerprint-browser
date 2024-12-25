-- Add migration script here
create table environment_transfer_history
(

    id             INTEGER
        primary key autoincrement,
    environment_id TEXT not null,
    from_user_uuid TEXT not null,
    to_user_uuid        TEXT not null,
    created_at     DATETIME default CURRENT_TIMESTAMP,
    updated_at     DATETIME default CURRENT_TIMESTAMP,
    deleted_at     DATETIME,
    foreign key (environment_id) references environments (id),
    foreign key (from_user_uuid) references users (uuid)
    foreign key (to_user_uuid) references users (uuid)
);
