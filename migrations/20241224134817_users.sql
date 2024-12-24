-- Add migration script here
create table users
(
    id             INTEGER
        primary key autoincrement,
    uuid           integer not null unique,
    user_info_id   integer not null,
    user_avatar_id integer not null,
    created_at     DATETIME default CURRENT_TIMESTAMP,
    updated_at     DATETIME default CURRENT_TIMESTAMP,
    deleted_at     DATETIME,
    foreign key (user_info_id) references user_infos (id),
    foreign key (user_avatar_id) references user_avatars (id)
);
