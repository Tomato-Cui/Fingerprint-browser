-- Add migration script here
CREATE TABLE
    team_extension_relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        extension_uuid TEXT NOT NULL,
        team_id INTEGER,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        deleted_at    DATETIME,
        FOREIGN KEY (extension_uuid) REFERENCES extensions (uuid),
        FOREIGN KEY (team_id) REFERENCES teams (id)
    );