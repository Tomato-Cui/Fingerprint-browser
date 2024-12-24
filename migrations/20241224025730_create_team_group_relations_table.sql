-- Add migration script here
create table
    user_team_group_relation
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    team_id       INTEGER,
    team_group_id INTEGER,
    created_at    DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at    DATETIME,
    FOREIGN KEY (team_id) REFERENCES teams (id),
    FOREIGN KEY (team_group_id) REFERENCES team_groups (id)
);