-- Add migration script here
CREATE TABLE
    user_group_relations
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT, 
    user_id    TEXT NOT NULL,              
    group_id   TEXT NOT NULL,                     
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME,
    FOREIGN KEY (group_id) REFERENCES groups (id),
    FOREIGN KEY (user_id) REFERENCES users (id)  
);
