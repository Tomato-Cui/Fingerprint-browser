-- Add migration script here
CREATE TABLE
    environment_extension_relation (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        environment_uuid TEXT NOT NULL,
        extension_uuid TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        deleted_at DATETIME,
        FOREIGN KEY (environment_uuid) REFERENCES environments (uuid),
        FOREIGN KEY (extension_uuid) REFERENCES extensions (uuid),
        CONSTRAINT unique_environmnet_extension UNIQUE (environment_uuid, extension_uuid)
    );