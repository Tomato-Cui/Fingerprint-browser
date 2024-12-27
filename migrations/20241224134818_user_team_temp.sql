-- Add migration script here

create table user_team_temps
(
    id            INTEGER
        primary key autoincrement,
    user_uuid     TEXT not null,
    team_id       integer not null,
    allow_1       integer,
    allow_2       INTEGER,
    description   TEXT,
    created_at    DATETIME default CURRENT_TIMESTAMP,
    updated_at    DATETIME default CURRENT_TIMESTAMP,
    deleted_at    DATETIME,
    foreign key (user_uuid) references users (uuid),
    foreign key (team_id) references teams (id)
);