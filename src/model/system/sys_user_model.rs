use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AddSysUser {
    pub mobile: String,             //手机
    pub user_name: String,          //姓名
    pub password: Option<String>,   //密码
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: Option<String>,     //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}

#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UpdateSysUser {
    pub id: i64,                    //主键
    pub mobile: String,             //手机
    pub user_name: String,          //姓名
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: Option<String>,     //备注
    pub create_time: NaiveDateTime, //创建时间, //创建时间
    pub update_time: NaiveDateTime, //修改时间, //修改时间
}

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    QueryableByName,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUser {
    pub id: i64,                    //主键
    pub mobile: String,             //手机
    pub user_name: String,          //姓名
    pub password: String,           //密码
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: Option<String>,     //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}
