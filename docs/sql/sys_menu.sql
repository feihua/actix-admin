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
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (3, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 0, 3, 2, '用户管理', '/system/user/list', '/api/user_list', ' ', '用户管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (4, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 2, 2, '角色管理', '/system/role/list', '/api/role_list', ' ', '角色管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (5, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 2, '菜单管理', '/system/menu/list', '/api/menu_list', ' ', '菜单管理', 2);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (6, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 3, '更新用户状态接口', '', '/api/update_user_status', ' ', '更新用户状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (7, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 3, '保存用户弹窗', '', '/api/user_save_view', ' ', '保存用户弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (8, '2022-07-14 17:40:10', '2022-07-14 17:40:10', 1, 1, 3, '保存用户接口', '', '/api/user_save', ' ', '保存用户接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (9, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '删除用户接口', '', '/api/user_delete', ' ', '删除用户接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (10, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '更新用户弹窗', '', '/api/user_update_view', ' ', '更新用户弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (11, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '更新用户接口', '', '/api/user_update', ' ', '更新用户接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (12, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '更新用户密码弹窗', '', '/api/update_user_password_view', ' ', '更新用户密码弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (13, '2022-07-14 17:40:11', '2022-07-26 21:58:51', 1, 1, 3, '更新用户密码', '', '/api/update_user_password', ' ', '更新用户密码接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (14, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '设置角色弹窗', '', '/api/update_user_role_view', ' ', '设置角色弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (15, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '保存用户角色', '', '/api/update_user_role', ' ', '保存用户角色接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (16, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '用户关联的角色', '', '/api/query_user_role', ' ', '获取用户关联的角色', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (17, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 3, '查询用户菜单接口', '', '/api/query_user_menu', ' ', '查询用户菜单接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (18, '2022-07-14 17:40:11', '2022-07-14 17:40:11', 1, 1, 4, '更新角色状态接口', '', '/api/update_role_status', ' ', '更新角色状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (19, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '保存角色弹窗', '', '/api/role_save_view', ' ', '保存角色弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (20, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '保存角色接口', '', '/api/role_save', ' ', '保存角色接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (21, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '删除角色接口', '', '/api/role_delete', ' ', '删除角色接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (22, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '修改角色弹窗', '', '/api/role_update_view', ' ', '修改角色弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (23, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '更新角色接口', '', '/api/role_update', ' ', '更新角色接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (24, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 0, 1, 4, '设置权限弹窗', '', '/api/query_role_menu_view', ' ', '设置权限弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (25, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '菜单角色关联', '', '/api/query_role_menu', ' ', '菜单角色关联', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (26, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 4, '保存角色菜单关联', '', '/api/update_role_menu', ' ', '角色菜单关联接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (27, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '更新菜单状态接口', '', '/api/update_menu_status', ' ', '更新菜单状态接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (28, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '保存菜单弹窗', '', '/api/menu_save_view', ' ', '保存菜单弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (29, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '保存菜单接口', '', '/api/menu_save', ' ', '保存菜单接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (30, '2022-07-14 17:40:12', '2022-07-14 17:40:12', 1, 1, 5, '删除菜单接口', '', '/api/menu_delete', ' ', '删除菜单接口', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (31, '2022-07-14 17:40:13', '2022-07-14 17:40:13', 1, 1, 5, '修改菜单弹窗', '', '/api/menu_update_view', ' ', '修改菜单弹窗', 3);
INSERT INTO sys_menu (id, gmt_create, gmt_modified, status_id, sort, parent_id, menu_name, menu_url, api_url, menu_icon, remark, menu_type) VALUES (32, '2022-07-14 17:40:13', '2022-07-14 17:40:13', 1, 1, 5, '更新菜单接口', '', '/api/menu_update', ' ', '更新菜单接口', 3);
