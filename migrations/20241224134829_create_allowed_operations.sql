-- Add migration script here
CREATE TABLE allowed_operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 主键，自增
    operation_name TEXT NOT NULL,         -- 操作名称（例如：create_user, update_product）
    operation_description TEXT,           -- 操作描述
    permission_id integer,                -- 操作类型
    resource_name TEXT NOT NULL,          -- 资源名称（例如：users, products）
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,  -- 更新时间
    FOREIGN KEY (permission_id) REFERENCES team_group_permission (id)
);

-- 插入用户相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/login+POST', '用户登录', 4, '/api/v1/users/login+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/logout+GET', '用户登出', 1, '/api/v1/users/logout+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/register+POST', '用户注册', 4, '/api/v1/users/register+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/search-by-email+POST', '通过邮箱搜索用户', 1, '/api/v1/users/search-by-email+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/reset-password+POST', '重置密码', 2, '/api/v1/users/reset-password+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/users/register/send+POST', '发送注册请求', 4, '/api/v1/users/register/send+POST');

-- 插入团队相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams+GET', '查询团队', 1, '/api/v1/teams+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/id+GET', '通过ID查询团队', 1, '/api/v1/teams/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/is-leader+GET', '检查是否为团队领导', 1, '/api/v1/teams/is-leader+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/search-by-name+GET', '通过名称搜索团队', 1, '/api/v1/teams/search-by-name+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/current+GET', '查询当前团队', 1, '/api/v1/teams/current+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/all-user+POST', '查询团队所有用户', 1, '/api/v1/teams/all-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/all-blocked-user+POST', '查询团队所有被屏蔽用户', 1, '/api/v1/teams/all-blocked-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/group/all-user+POST', '查询团队分组所有用户', 1, '/api/v1/teams/group/all-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/blocked+PUT', '屏蔽用户', 2, '/api/v1/teams/blocked+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/un-blocked+PUT', '解除屏蔽用户', 2, '/api/v1/teams/un-blocked+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/create+POST', '创建团队', 4, '/api/v1/teams/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams+PUT', '修改团队', 2, '/api/v1/teams+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/switch+PUT', '切换团队', 2, '/api/v1/teams/switch+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/remove-user+PUT', '移除当前用户', 2, '/api/v1/teams/remove-user+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/user-info+PUT', '修改团队用户信息', 2, '/api/v1/teams/user-info+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams+DELETE', '删除团队', 3, '/api/v1/teams+DELETE');

-- 插入团队分组相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/team-groups/id+GET', '通过ID查询团队分组', 1, '/api/v1/team-groups/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/team-groups+POST', '查询所有团队分组', 1, '/api/v1/team-groups+POST');

-- 插入消息相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/receive-query+POST', '查询用户接收的消息', 1, '/api/v1/messages/user/receive-query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/receive-query+POST', '查询团队接收的消息', 1, '/api/v1/messages/team/receive-query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/send+POST', '发送团队消息', 4, '/api/v1/messages/team/send+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/send+POST', '发送用户消息', 4, '/api/v1/messages/user/send+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/reject+POST', '拒绝消息', 2, '/api/v1/messages/reject+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/allow+PUT', '允许团队消息', 2, '/api/v1/messages/team/allow+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/allow+PUT', '允许用户消息', 2, '/api/v1/messages/user/allow+PUT');

-- 插入扩展相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/user/create+POST', '创建用户扩展', 4, '/api/v1/extensions/user/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/team/create+POST', '创建团队扩展', 4, '/api/v1/extensions/team/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/user+POST', '通过用户查询扩展', 1, '/api/v1/extensions/query/user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/team+POST', '通过团队查询扩展', 1, '/api/v1/extensions/query/team+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query+POST', '查询扩展', 1, '/api/v1/extensions/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/environment+POST', '通过环境查询扩展', 1, '/api/v1/extensions/query/environment+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/environment/use+POST', '使用环境扩展', 2, '/api/v1/extensions/environment/use+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/environment/remove+DELETE', '移除环境扩展', 3, '/api/v1/extensions/environment/remove+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/update+PUT', '更新扩展', 2, '/api/v1/extensions/update+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/user/toggle-extension+PUT', '切换用户扩展', 2, '/api/v1/extensions/user/toggle-extension+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/delete/uuid+DELETE', '通过UUID删除扩展', 3, '/api/v1/extensions/delete/uuid+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/remove/user-uuid+DELETE', '通过用户UUID移除扩展', 3, '/api/v1/extensions/remove/user-uuid+DELETE');

-- 插入环境相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/uuid+GET', '通过UUID查询环境', 1, '/api/v1/environments/uuid+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments+GET', '查询环境', 1, '/api/v1/environments+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/group+GET', '通过分组查询环境', 1, '/api/v1/environments/group+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/team+GET', '通过团队查询环境', 1, '/api/v1/environments/team+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/extension+POST', '通过扩展查询环境', 1, '/api/v1/environments/extension+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/create+POST', '创建环境', 4, '/api/v1/environments/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/detail/create+POST', '创建环境详情', 4, '/api/v1/environments/detail/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch+POST', '批量创建环境', 4, '/api/v1/environments/batch+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch-import+POST', '批量导入环境', 4, '/api/v1/environments/batch-import+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch-export+POST', '批量导出环境', 1, '/api/v1/environments/batch-export+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/basic+PUT', '修改环境基础信息', 2, '/api/v1/environments/basic+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments+PUT', '修改环境信息', 2, '/api/v1/environments+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/proxy+PUT', '修改环境代理', 2, '/api/v1/environments/proxy+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/move-to-group+PUT', '将环境移动到分组', 2, '/api/v1/environments/move-to-group+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch/move-to-group+PUT', '批量将环境移动到分组', 2, '/api/v1/environments/batch/move-to-group+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments+DELETE', '删除环境', 3, '/api/v1/environments+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch+DELETE', '批量删除环境', 3, '/api/v1/environments/batch+DELETE');

-- 插入环境回收站相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/uuid+GET', '通过UUID查询环境回收站', 1, '/api/v1/environment-trash/uuid+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash+GET', '查询环境回收站', 1, '/api/v1/environment-trash+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recover+PUT', '恢复环境', 2, '/api/v1/environment-trash/recover+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recovers+PUT', '批量恢复环境', 2, '/api/v1/environment-trash/recovers+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recover-all+PUT', '恢复所有环境', 2, '/api/v1/environment-trash/recover-all+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/batch+DELETE', '批量删除环境回收站', 3, '/api/v1/environment-trash/batch+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/clean+DELETE', '清空环境回收站', 3, '/api/v1/environment-trash/clean+DELETE');

-- 插入环境转移历史相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-transfer-historys/id+GET', '通过ID查询环境转移历史', 1, '/api/v1/environment-transfer-historys/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-transfer-historys+GET', '查询环境转移历史', 1, '/api/v1/environment-transfer-historys+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-transfer-historys/batch+POST', '批量创建环境转移历史', 4, '/api/v1/environment-transfer-historys/batch+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-transfer-historys+DELETE', '通过UUID删除环境转移历史', 3, '/api/v1/environment-transfer-historys+DELETE');

-- 插入环境代理组相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxy-groups/id+GET', '通过ID查询环境代理组', 1, '/api/v1/environment-proxy-groups/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxy-groups+GET', '查询环境代理组', 1, '/api/v1/environment-proxy-groups+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxy-groups+POST', '创建环境代理组', 4, '/api/v1/environment-proxy-groups+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxy-groups+PUT', '修改环境代理组', 2, '/api/v1/environment-proxy-groups+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxy-groups+DELETE', '删除环境代理组', 3, '/api/v1/environment-proxy-groups+DELETE');

-- 插入环境代理相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/id+GET', '通过ID查询环境代理', 1, '/api/v1/environment-proxies/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies+GET', '查询环境代理', 1, '/api/v1/environment-proxies+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/group+GET', '通过分组查询环境代理', 1, '/api/v1/environment-proxies/group+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies+POST', '创建环境代理', 4, '/api/v1/environment-proxies+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies+PUT', '修改环境代理', 2, '/api/v1/environment-proxies+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies+DELETE', '删除环境代理', 3, '/api/v1/environment-proxies+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/batch+DELETE', '批量删除环境代理', 3, '/api/v1/environment-proxies/batch+DELETE');

-- 插入环境分组相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-groups/id+GET', '通过ID查询环境分组', 1, '/api/v1/environment-groups/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-groups+GET', '查询环境分组', 1, '/api/v1/environment-groups+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-groups+POST', '创建环境分组', 4, '/api/v1/environment-groups+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-groups+PUT', '修改环境分组', 2, '/api/v1/environment-groups+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-groups+DELETE', '删除环境分组', 3, '/api/v1/environment-groups+DELETE');

-- 插入环境指纹相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/id+GET', '通过ID查询环境指纹', 1, '/api/v1/environment-fingerprints/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints+GET', '查询环境指纹', 1, '/api/v1/environment-fingerprints+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints+POST', '创建环境指纹', 4, '/api/v1/environment-fingerprints+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints+PUT', '修改环境指纹', 2, '/api/v1/environment-fingerprints+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints+DELETE', '删除环境指纹', 3, '/api/v1/environment-fingerprints+DELETE');

-- 插入环境Cookie相关操作
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+GET', '通过环境查询Cookie', 1, '/api/v1/environment-cookies/environment+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+POST', '创建环境Cookie', 4, '/api/v1/environment-cookies/environment+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+PUT', '修改环境Cookie', 2, '/api/v1/environment-cookies/environment+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+DELETE', '删除环境Cookie', 3, '/api/v1/environment-cookies/environment+DELETE');