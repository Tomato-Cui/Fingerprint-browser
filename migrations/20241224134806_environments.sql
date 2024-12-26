-- Add migration script here
create table environments
(
    id           INTEGER
        primary key autoincrement,
    uuid         INTEGER unique,
    user_uuid    TEXT,
    team_id      integer,
    proxy_id     integer,
    fp_info_id   integer,
    group_id     integer,
    name         TEXT               not null,
    description  TEXT,
    default_urls TEXT,
    proxy_enable INTEGER  default 0 not null,
    status       INTEGER  default 1 not null,
    created_at   DATETIME default CURRENT_TIMESTAMP,
    updated_at   DATETIME default CURRENT_TIMESTAMP,
    lasted_at    DATETIME default CURRENT_TIMESTAMP,
    deleted_at   DATETIME,
    foreign key (user_uuid) references users (uuid),
    foreign key (team_id) references teams (id),
    foreign key (proxy_id) references environment_proxies(id),
    foreign key (fp_info_id) references environment_fingerprints(id),
    foreign key (group_id) references environment_groups(id)
);