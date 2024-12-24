-- Add migration script here
create table
    environment_transfers
(
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    team_id        Integer,
    environment_id integer,
    teak_over      INTEGER,
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at     DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at     DATETIME,
    FOREIGN KEY (team_id) REFERENCES teams (id),
    FOREIGN KEY (environment_id) REFERENCES environments (id)
);