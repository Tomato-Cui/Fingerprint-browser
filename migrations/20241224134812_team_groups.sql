-- Add migration script here
create table team_groups
(
    id                       INTEGER
        primary key autoincrement,
    name                     TEXT,
    description              INTEGER,
    team_id                  integer,
    team_group_permission_id integer,
    created_at               DATETIME default CURRENT_TIMESTAMP,
    updated_at               DATETIME default CURRENT_TIMESTAMP,
    deleted_at               DATETIME,
    foreign key (team_id) references teams (id),
    foreign key (team_group_permission_id) references team_group_permission (id)
);