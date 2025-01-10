-- Add migration script here
CREATE TABLE
    operation_logs (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 主键，自增
        team_id INTEGER,
        user_uuid INTEGER,
        allowed_operation_id INTEGER NOT NULL, -- 操作 ID（关联 allowed_operations 表）
        operation_target TEXT, -- 操作目标（例如：具体的资源 ID）
        operation_status INTEGER, -- 操作状态（例如：success, failed）
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 更新时间
        deleted_at DATETIME, -- 删除时间（软删除）
        FOREIGN KEY (allowed_operation_id) REFERENCES allowed_operations (id), -- 外键，关联 allowed_operations 表
        FOREIGN KEY (user_uuid) REFERENCES users (uuid), -- 外键，关联 allowed_operations 表
        FOREIGN KEY (team_id) REFERENCES teams (id) -- 外键，关联 allowed_operations 表
    );