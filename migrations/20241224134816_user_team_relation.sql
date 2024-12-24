-- Add migration script here
create table user_team_relation
(
    id            INTEGER
        primary key autoincrement,
    user_uuid     TEXT not null,
    team_id       TEXT not null,
    send_allow    INTEGER,
    receive_allow INTEGER,
    blocked       INTEGER,
    description   TEXT,
    created_at    DATETIME default CURRENT_TIMESTAMP,
    updated_at    DATETIME default CURRENT_TIMESTAMP,
    deleted_at    DATETIME,
    foreign key (user_uuid) references users (uuid),
    foreign key (team_id) references teams (id)
);

