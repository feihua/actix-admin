use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Serialize, Deserialize, QueryableByName, AsChangeset)]
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysMenu {
    pub id: i64,
    pub menu_name: String,
    pub menu_type: i8,
    pub status_id: i8,
    pub sort: i32,
    pub parent_id: i64,
    pub menu_url: String,
    pub api_url: String,
    pub menu_icon: Option<String>,
    pub remark: Option<String>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,

}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysMenuAdd {
    pub menu_name: String,
    pub menu_type: i8,
    pub status_id: i8,
    pub sort: i32,
    pub parent_id: i64,
    pub menu_url: String,
    pub api_url: String,
    pub menu_icon: Option<String>,
    pub remark: Option<String>,

}

#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysMenuUpdate {
    pub id: i64,
    pub menu_name: String,
    pub menu_type: i8,
    pub status_id: i8,
    pub sort: i32,
    pub parent_id: i64,
    pub menu_url: String,
    pub api_url: String,
    pub menu_icon: Option<String>,
    pub remark: Option<String>,

}

#[derive(QueryableByName)]
pub struct StringColumn {
    #[sql_type = "diesel::sql_types::Varchar"]
    pub api_url: String,
}