use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_user_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AddSysUserRole {
    pub user_id: i64,               //用户ID
    pub role_id: i64,               //角色ID
    pub create_time: NaiveDateTime, //创建时间
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
#[diesel(table_name = crate::schema::sys_user_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserRole {
    pub id: i64,                    //主键
    pub user_id: i64,               //用户ID
    pub role_id: i64,               //角色ID
    pub create_time: NaiveDateTime, //创建时间
}
