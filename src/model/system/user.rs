use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};



#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserAdd {
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,

}


#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUserUpdate {
    pub id: i64,
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,

}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Serialize, Deserialize, QueryableByName, AsChangeset)]
#[diesel(table_name = crate::schema::sys_user)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysUser {
    pub id: i64,
    pub mobile: String,
    pub user_name: String,
    pub password: String,
    pub status_id: i32,
    pub sort: i32,
    pub remark: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,

}

impl SysUser {

}
