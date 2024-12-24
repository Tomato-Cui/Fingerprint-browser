-- Add migration script here
create table environment_transfer_history
(

    id             INTEGER
        primary key autoincrement,
    team_id        TEXT not null,
    environment_id TEXT not null,
    from_user_uuid TEXT,
    created_at     DATETIME default CURRENT_TIMESTAMP,
    updated_at     DATETIME default CURRENT_TIMESTAMP,
    deleted_at     DATETIME,
    foreign key (team_id) references teams (id),
    foreign key (environment_id) references environments (id),
    foreign key (from_user_uuid) references users (uuid)
);
