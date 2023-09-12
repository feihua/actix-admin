-- Your SQL goes here
create table sys_user
(
    id          bigint auto_increment comment '主键'
        primary key,
    mobile      varchar(11) default ''                not null comment '手机',
    user_name   varchar(50)                           not null comment '姓名',
    password    varchar(64) charset utf8mb3           not null comment '密码',
    status_id   int         default 1                 not null comment '状态(1:正常，0:禁用)',
    sort        int         default 1                 not null comment '排序',
    remark      varchar(255)                          null comment '备注',
    create_time datetime    default CURRENT_TIMESTAMP not null comment '创建时间',
    update_time datetime    default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '修改时间',
    constraint AK_phone
        unique (mobile)
)
    comment '用户信息';

INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time)
VALUES (1, '18613030352', '超级管理员', '123456', 1, 1, '超级管理员', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO sys_user (id, mobile, user_name, password, status_id, sort, remark, create_time, update_time)
VALUES (2, '18613030351', '普通用户', '123456', 1, 1, '演示权限', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
