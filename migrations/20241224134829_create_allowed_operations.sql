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


-- Teams
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query+POST', '查询团队', 4, '/api/v1/teams/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/id+POST', '通过ID查询团队', 4, '/api/v1/teams/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/is-leader+POST', '查询是否为团队领导', 4, '/api/v1/teams/query/is-leader+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/search-by-name+POST', '通过名称搜索团队', 4, '/api/v1/teams/query/search-by-name+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/current+POST', '查询当前团队', 4, '/api/v1/teams/query/current+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/all-user+POST', '查询团队所有用户', 4, '/api/v1/teams/query/all-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/all-blocked-user+POST', '查询团队所有被屏蔽用户', 4, '/api/v1/teams/query/all-blocked-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/query/group/all-user+POST', '查询团队组所有用户', 4, '/api/v1/teams/query/group/all-user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/blocked+PUT', '屏蔽用户', 4, '/api/v1/teams/blocked+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/un-blocked+PUT', '解除屏蔽用户', 4, '/api/v1/teams/un-blocked+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/create+POST', '创建团队', 4, '/api/v1/teams/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/+PUT', '修改团队', 4, '/api/v1/teams/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/switch+PUT', '切换团队', 4, '/api/v1/teams/switch+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/remove-user+PUT', '移除当前用户', 4, '/api/v1/teams/remove-user+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/user-info+PUT', '修改团队用户信息', 4, '/api/v1/teams/user-info+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/teams/+DELETE', '删除团队', 4, '/api/v1/teams/+DELETE');

-- Team Groups
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/team-groups/id+GET', '通过ID查询团队组', 4, '/api/v1/team-groups/id+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/team-groups/+POST', '查询所有团队组', 4, '/api/v1/team-groups/+POST');

-- Messages
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/receive-query+POST', '查询用户接收的消息', 4, '/api/v1/messages/user/receive-query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/receive-query+POST', '查询团队接收的消息', 4, '/api/v1/messages/team/receive-query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/send+POST', '发送团队消息', 4, '/api/v1/messages/team/send+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/send+POST', '发送用户消息', 4, '/api/v1/messages/user/send+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/reject+POST', '拒绝消息', 4, '/api/v1/messages/reject+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/team/allow+PUT', '允许团队消息', 4, '/api/v1/messages/team/allow+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/messages/user/allow+PUT', '允许用户消息', 4, '/api/v1/messages/user/allow+PUT');

-- Extensions
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/user/create+POST', '创建用户扩展', 4, '/api/v1/extensions/user/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/team/create+POST', '创建团队扩展', 4, '/api/v1/extensions/team/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/user+POST', '查询用户扩展', 4, '/api/v1/extensions/query/user+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/team+POST', '查询团队扩展', 4, '/api/v1/extensions/query/team+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query+POST', '查询扩展', 4, '/api/v1/extensions/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/query/environment+POST', '查询环境扩展', 4, '/api/v1/extensions/query/environment+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/environmnet/use+POST', '使用环境扩展', 4, '/api/v1/extensions/environmnet/use+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/environmnet/remove+DELETE', '移除环境扩展', 4, '/api/v1/extensions/environmnet/remove+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/update+PUT', '更新扩展', 4, '/api/v1/extensions/update+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/user/toggle-extension+PUT', '切换用户扩展', 4, '/api/v1/extensions/user/toggle-extension+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/delete/uuid+DELETE', '通过UUID删除扩展', 4, '/api/v1/extensions/delete/uuid+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/extensions/remove/user-uuid+DELETE', '通过用户UUID移除扩展', 4, '/api/v1/extensions/remove/user-uuid+DELETE');

-- Environments
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/query/uuid+POST', '通过UUID查询环境', 4, '/api/v1/environments/query/uuid+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/query+POST', '查询环境', 4, '/api/v1/environments/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/query/group+POST', '通过组查询环境', 4, '/api/v1/environments/query/group+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/query/team+POST', '通过团队查询环境', 4, '/api/v1/environments/query/team+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/query/extension+POST', '通过扩展查询环境', 4, '/api/v1/environments/query/extension+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/create+POST', '创建环境', 4, '/api/v1/environments/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/detail/create+POST', '创建环境详情', 4, '/api/v1/environments/detail/create+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch+POST', '批量创建环境', 4, '/api/v1/environments/batch+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch-import+POST', '批量导入环境', 4, '/api/v1/environments/batch-import+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch-export+POST', '批量导出环境', 4, '/api/v1/environments/batch-export+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/basic+PUT', '修改环境基本信息', 4, '/api/v1/environments/basic+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/+PUT', '修改环境信息', 4, '/api/v1/environments/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/proxy+PUT', '修改环境代理', 4, '/api/v1/environments/proxy+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/move-to-group+PUT', '将环境移动到组', 4, '/api/v1/environments/move-to-group+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch/move-to-group+PUT', '批量将环境移动到组', 4, '/api/v1/environments/batch/move-to-group+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/+DELETE', '删除环境', 4, '/api/v1/environments/+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environments/batch+DELETE', '批量删除环境', 4, '/api/v1/environments/batch+DELETE');

-- Environment Trash
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/query/uuid+GET', '通过UUID查询环境回收站', 4, '/api/v1/environment-trash/query/uuid+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/query+POST', '查询环境回收站', 4, '/api/v1/environment-trash/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recover+PUT', '恢复环境', 4, '/api/v1/environment-trash/recover+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recovers+PUT', '批量恢复环境', 4, '/api/v1/environment-trash/recovers+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/recover-all+PUT', '恢复所有环境', 4, '/api/v1/environment-trash/recover-all+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/batch+DELETE', '批量删除环境回收站', 4, '/api/v1/environment-trash/batch+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-trash/clean+DELETE', '清空环境回收站', 4, '/api/v1/environment-trash/clean+DELETE');

-- Environment Transfer History
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-transfer-historys/query/id+POST', '通过ID查询环境转移历史', 4, '/api/v1/environmnet-transfer-historys/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-transfer-historys/query+POST', '查询环境转移历史', 4, '/api/v1/environmnet-transfer-historys/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-transfer-historys/batch+POST', '批量创建环境转移历史', 4, '/api/v1/environmnet-transfer-historys/batch+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-transfer-historys/+DELETE', '通过UUID删除环境转移历史', 4, '/api/v1/environmnet-transfer-historys/+DELETE');

-- Environment Proxy Groups
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-proxy-groups/query/id+POST', '通过ID查询环境代理组', 4, '/api/v1/environmnet-proxy-groups/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-proxy-groups/query+POST', '查询环境代理组', 4, '/api/v1/environmnet-proxy-groups/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-proxy-groups/+POST', '创建环境代理组', 4, '/api/v1/environmnet-proxy-groups/+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-proxy-groups/+PUT', '修改环境代理组', 4, '/api/v1/environmnet-proxy-groups/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-proxy-groups/+DELETE', '删除环境代理组', 4, '/api/v1/environmnet-proxy-groups/+DELETE');

-- Environment Proxies
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/query/id+POST', '通过ID查询环境代理', 4, '/api/v1/environment-proxies/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/query+POST', '查询环境代理', 4, '/api/v1/environment-proxies/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/query/group+POST', '通过组查询环境代理', 4, '/api/v1/environment-proxies/query/group+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/+POST', '创建环境代理', 4, '/api/v1/environment-proxies/+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/+PUT', '修改环境代理', 4, '/api/v1/environment-proxies/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/+DELETE', '删除环境代理', 4, '/api/v1/environment-proxies/+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-proxies/batch+DELETE', '批量删除环境代理', 4, '/api/v1/environment-proxies/batch+DELETE');

-- Environment Groups
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-groups/query/id+POST', '通过ID查询环境组', 4, '/api/v1/environmnet-groups/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-groups/query+POST', '查询环境组', 4, '/api/v1/environmnet-groups/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-groups/+POST', '创建环境组', 4, '/api/v1/environmnet-groups/+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-groups/+PUT', '修改环境组', 4, '/api/v1/environmnet-groups/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environmnet-groups/+DELETE', '删除环境组', 4, '/api/v1/environmnet-groups/+DELETE');

-- Environment Fingerprints
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/query/id+POST', '通过ID查询环境指纹', 4, '/api/v1/environment-fingerprints/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/query+POST', '查询环境指纹', 4, '/api/v1/environment-fingerprints/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/+POST', '创建环境指纹', 4, '/api/v1/environment-fingerprints/+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/+PUT', '修改环境指纹', 4, '/api/v1/environment-fingerprints/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-fingerprints/+DELETE', '删除环境指纹', 4, '/api/v1/environment-fingerprints/+DELETE');

-- Environment Cookies
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment/query/id+POST', '通过ID查询环境Cookie', 4, '/api/v1/environment-cookies/environment/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+POST', '创建环境Cookie', 4, '/api/v1/environment-cookies/environment+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+PUT', '修改环境Cookie', 4, '/api/v1/environment-cookies/environment+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-cookies/environment+DELETE', '删除环境Cookie', 4, '/api/v1/environment-cookies/environment+DELETE');

-- Environment Accounts
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/query/id+POST', '通过ID查询环境账户', 4, '/api/v1/environment-accounts/query/id+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/query+POST', '查询环境账户', 4, '/api/v1/environment-accounts/query+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/user+GET', '查询当前用户的环境账户', 4, '/api/v1/environment-accounts/user+GET');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/+POST', '创建环境账户', 4, '/api/v1/environment-accounts/+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/+PUT', '修改环境账户', 4, '/api/v1/environment-accounts/+PUT');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/+DELETE', '删除环境账户', 4, '/api/v1/environment-accounts/+DELETE');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/environment-accounts/batch+DELETE', '批量删除环境账户', 4, '/api/v1/environment-accounts/batch+DELETE');

-- Browsers
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/browsers/start+POST', '启动浏览器', 4, '/api/v1/browsers/start+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/browsers/starts+POST', '批量启动浏览器', 4, '/api/v1/browsers/starts+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/browsers/stops+POST', '停止浏览器', 4, '/api/v1/browsers/stops+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/browsers/stop-all+POST', '停止所有浏览器', 4, '/api/v1/browsers/stop-all+POST');
INSERT INTO allowed_operations (operation_name, operation_description, permission_id, resource_name) VALUES ('/api/v1/browsers/status+GET', '查询浏览器状态', 4, '/api/v1/browsers/status+GET');