-- Add migration script here
create table resource_whitelist
(
    id          INTEGER
        primary key autoincrement,
    path        TEXT not null,
    method      TEXT not null,
    description TEXT,
    can_use     INTEGER,
    created_at  DATETIME default CURRENT_TIMESTAMP,
    updated_at  DATETIME default CURRENT_TIMESTAMP,
    deleted_at  DATETIME
);


INSERT INTO resource_whitelist (path, method, description, can_use) VALUES
('/api/v1/users/login', 'POST', '用户登录', 1),
('/api/v1/users/register', 'POST', '用户注册', 1),
('/api/v1/users/reset-password', 'POST', '重置密码', 1),
('/api/v1/users/register/send', 'POST', '发送注册验证码', 1),
('/api/v1/status', 'GET', '查看状态', 1);


