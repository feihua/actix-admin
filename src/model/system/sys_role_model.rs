use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AddSysRole {
    pub role_name: String,          //名称
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: String,             //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}

#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UpdateSysRole {
    pub id: i64,                    //主键
    pub role_name: String,          //名称
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: String,             //备注
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
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysRole {
    pub id: i64,                    //主键
    pub role_name: String,          //名称
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub remark: String,             //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}
