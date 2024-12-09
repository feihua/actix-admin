use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Serialize, Deserialize, QueryableByName, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserRole {
    pub id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub status_id: i8,
    pub sort: i32,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,

}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_user_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserRoleAdd {
    pub user_id: i64,
    pub role_id: i64,
    pub status_id: i8,
    pub sort: i32,

}
