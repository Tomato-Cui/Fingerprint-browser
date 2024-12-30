-- Add migration script here
CREATE TABLE
    user_extension_relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_uuid TEXT NOT NULL,
        extension_uuid TEXT NOT NULL,
        open INTEGER,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        deleted_at DATETIME,
        FOREIGN KEY (user_uuid) REFERENCES users (uuid),
        FOREIGN KEY (extension_uuid) REFERENCES extensions (uuid),
        CONSTRAINT unique_environment_extension UNIQUE (user_uuid, extension_uuid)
    );