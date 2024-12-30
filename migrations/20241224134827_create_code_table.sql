--  Add migration script here
create table
    codes (
        id INTEGER primary key autoincrement,
        kind integer not null,
        key text not null,
        value text not null,
        expired_at TEXT not null,
        created_at DATETIME default CURRENT_TIMESTAMP,
        updated_at DATETIME default CURRENT_TIMESTAMP,
        deleted_at DATETIM
    );