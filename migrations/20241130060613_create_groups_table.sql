-- Groups Table
CREATE TABLE
    groups (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE, -- 分组名称
        description TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 更新时间
        deleted_at DATETIME -- 删除时间
    );

INSERT INTO
    groups (name, description)
VALUES
    ('', '');