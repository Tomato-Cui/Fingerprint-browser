-- Add migration script here
create table
    resource_whitelist
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    href        TEXT not null,
    method      TEXT not null,
    description TEXT,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);

INSERT INTO resource_whitelist (href, method, description)
VALUES
    ('/api/v1/status', 'GET', '获取服务状态'),
    ('/api/v1/users/login', 'POST', '用户登录'),
    ('/api/v1/users/register', 'POST', '用户注册'),
    ('/api/v1/users/reset-password', 'POST', '用户重置密码');