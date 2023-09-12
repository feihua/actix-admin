-- Your SQL goes here
create table sys_user_role
(
    id          bigint auto_increment comment '主键'
        primary key,
    user_id     bigint   default 0                 not null comment '用户ID',
    role_id     bigint                             not null comment '角色ID',
    status_id   tinyint  default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int      default 1                 not null comment '排序',
    create_time datetime default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间'
)
    comment '角色用户关联表';

INSERT INTO sys_user_role (id, user_id, role_id, status_id, sort, create_time, update_time)
VALUES (1, 1, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
