create table sys_menu
(
    id           bigint unsigned auto_increment comment '主键'
        primary key,
    gmt_create   datetime         default CURRENT_TIMESTAMP not null comment '创建时间',
    gmt_modified datetime         default CURRENT_TIMESTAMP not null comment '修改时间',
    status_id    tinyint unsigned default 1                 not null comment '状态(1:正常，0:禁用)',
    sort         int unsigned     default 1                 not null comment '排序',
    parent_id    bigint unsigned                            not null comment '父ID',
    menu_name    varchar(50)                                not null comment '菜单名称',
    menu_url     varchar(255)     default ''                null comment '路由路径',
    api_url      varchar(255)     default ''                null comment '接口URL',
    menu_icon    varchar(255)                               null comment '菜单图标',
    remark       varchar(255)                               null comment '备注',
    menu_type    tinyint          default 1                 not null comment '菜单类型(1：目录   2：菜单   3：按钮)'
)
    comment '菜单信息' charset = utf8mb4;

INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (1, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 100, 0, '首页', '/welcome', '', 'SmileOutlined', '首页', 1);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (2, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 0, '权限管理', '/system', '', 'SettingOutlined', '权限管理', 1);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (3, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 0, 3, 2, '用户管理', '/system/user/list', '/api/system/user/list', ' ', '用户管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (4, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 2, 2, '角色管理', '/system/role/list', '/api/system/role/list', ' ', '角色管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (5, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 2, '菜单管理', '/system/menu/list', '/api/system/menu/list', ' ', '菜单管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (6, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 3, '添加', '', '/api/system/user/updateStatus', ' ', '更新状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (7, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 3, '保存', '', '/api/system/user/save', ' ', '保存接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (8, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '删除', '', '/api/system/user/delete', ' ', '删除接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (9, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '修改', '', '/api/system/user/view', ' ', '修改弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (10, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '更新', '', '/api/system/user/update', ' ', '更新接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (11, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '更新密码弹窗', '', '/api/system/user/password', ' ', '更新密码弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (12, '2022-07-14 17:40:11', '2022-07-26 21:58:51', 1, 1, 3, '更新密码', '', '/api/system/user/updatePassword', ' ', '更新密码接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (13, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '设置角色', '', '/api/system/role/list', ' ', '设置角色弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (14, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '保存用户角色', '', '/api/system/role/userRoleSave', ' ', '保存用户角色接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (15, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '用户关联的角色', '', '/api/system/role/userRoleList', ' ', '获取用户关联的角色', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (16, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '列出用户', '', '/api/user/pc/user/list', ' ', '列出用户', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (17, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 4, '添加', '', '/api/system/role/updateStatus', ' ', '更新状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (18, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '保存', '', '/api/system/role/save', ' ', '保存接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (19, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '删除', '', '/api/system/role/delete', ' ', '删除接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (20, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '修改', '', '/api/system/role/view', ' ', '修改修改弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (21, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '更新', '', '/api/system/role/update', ' ', '更新接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (22, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 0, 1, 4, '设置权限', '', '/api/system/menu/list', ' ', '设置权限弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (23, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '菜单角色关联', '', '/api/system/menu/roleMenuList', ' ', '菜单角色关联', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (24, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '保存角色菜单关联', '', '/api/system/menu/roleMenuSave', ' ', '角色菜单关联接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (25, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '添加', '', '/api/system/menu/updateStatus', ' ', '更新状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (26, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '保存', '', '/api/system/menu/save', ' ', '保存接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (27, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '删除', '', '/api/system/menu/delete', ' ', '删除接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (28, '2022-07-14 17:40:13', '2022-07-14 17:40:13', 1, 1, 5, '修改', '', '/api/system/menu/view', ' ', '修改弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (29, '2022-07-14 17:40:13', '2022-07-14 17:40:13', 1, 1, 5, '更新', '', '/api/system/menu/update', ' ', '更新', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (30, '2022-07-15 09:18:47', '2022-07-15 16:16:23', 0, 2, 0, 'test1', 'test', '/api/role_list', 'FrownOutlined', '12', 1);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (31, '2022-07-15 09:18:47', '2022-07-15 16:16:23', 1, 2, 3, '枚举状态', '', '/api/system/enum/list', ' ', '枚举状态', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (32, '2022-07-25 15:46:42', '2022-07-25 15:59:26', 1, 1, 0, 'test2', 'test3', '/api/user_list', 'FrownOutlined', 'test', 1);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (34, '2022-07-25 15:46:42', '2022-07-25 15:59:26', 1, 1, 0, 'test4', 'test4', '/auth/ping', 'FrownOutlined', 'test', 1);
