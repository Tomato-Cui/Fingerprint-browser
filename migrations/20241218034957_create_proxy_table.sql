CREATE TABLE
    proxies (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,         -- 自增ID
    kind           TEXT NOT NULL,                             -- 代理类型
    value          TEXT NOT NULL,                             -- 代理值
    environment_id INTEGER,
    owner_id INTEGER,
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP,        -- 创建时间
    updated_at     DATETIME DEFAULT CURRENT_TIMESTAMP,        -- 更新时间
    deleted_at     DATETIME,                                  -- 删除时间
    FOREIGN KEY (environment_id) REFERENCES environments (id), --外键关联environments表
    FOREIGN KEY (owner_id) REFERENCES users (id)  --外键关联environments表
);