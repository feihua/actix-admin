-- Your SQL goes here
create table sys_role_menu
(
    id          bigint auto_increment comment '主键'
        primary key,
    role_id     bigint                             not null comment '角色ID',
    menu_id     bigint                             not null comment '菜单ID',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '菜单角色关联表';

