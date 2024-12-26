CREATE TABLE team_group_permission (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    description TEXT,
    action TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    deleted_at DATETIME
);

INSERT INTO team_group_permission (name, description, action) VALUES ('查看', '允许查看环境', 'view');
INSERT INTO team_group_permission (name, description, action) VALUES ('编辑', '允许编辑环境', 'edit');
INSERT INTO team_group_permission (name, description, action) VALUES ('删除', '允许删除环境', 'delete');
INSERT INTO team_group_permission (name, description, action) VALUES ('增加', '允许添加环境', 'add');