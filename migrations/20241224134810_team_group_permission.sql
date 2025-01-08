CREATE TABLE team_group_permission (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    description TEXT,
    action TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

INSERT INTO team_group_permission (name, description, action) VALUES ('查看', '查看权限', 'view');
INSERT INTO team_group_permission (name, description, action) VALUES ('编辑', '允许编辑', 'edit');
INSERT INTO team_group_permission (name, description, action) VALUES ('删除', '允许删除', 'delete');
INSERT INTO team_group_permission (name, description, action) VALUES ('增加', '允许添加', 'add');