use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Serialize, Deserialize, QueryableByName, AsChangeset)]
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysRole {
    pub id: i64,
    pub role_name: String,
    pub status_id: i8,
    pub sort: i32,
    pub remark: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysRoleAdd {
    pub role_name: String,
    pub status_id: i8,
    pub sort: i32,
    pub remark: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysRoleUpdate {
    pub id: i64,
    pub role_name: String,
    pub status_id: i8,
    pub sort: i32,
    pub remark: String,
}