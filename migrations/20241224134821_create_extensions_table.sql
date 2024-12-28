-- Add migration script here
CREATE TABLE
    extensions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        uuid TEXT NOT NULL UNIQUE,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        cover_url TEXT,
        release_url TEXT,
        location_url TEXT NOT NULL,
        size INTEGER,
        all_can_use INTEGER,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        deleted_at DATETIME
    );