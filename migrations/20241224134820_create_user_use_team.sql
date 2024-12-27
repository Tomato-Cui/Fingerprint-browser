-- Add migration script here
create table user_use_team(
      id         INTEGER primary key autoincrement,
      user_uuid  TEXT,
      team_id    INTEGER ,
      created_at DATETIME default CURRENT_TIMESTAMP,
      updated_at DATETIME default CURRENT_TIMESTAMP,
      deleted_at DATETIME,
      foreign key (user_uuid) references users (uuid),
      foreign key (team_id) references teams (id)
);