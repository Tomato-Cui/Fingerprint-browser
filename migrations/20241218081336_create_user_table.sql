-- Add migration script here
CREATE TABLE
    users
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT, -- id
    nickname   TEXT NOT NULL UNIQUE,              -- nickname
    password   TEXT NOT NULL,                     -- pwd
    email      TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);