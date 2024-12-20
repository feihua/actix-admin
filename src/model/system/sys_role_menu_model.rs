use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_role_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AddSysRoleMenu {
    pub role_id: i64,               //角色ID
    pub menu_id: i64,               //菜单ID
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
#[diesel(table_name = crate::schema::sys_role_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysRoleMenu {
    pub id: i64,                    //主键
    pub role_id: i64,               //角色ID
    pub menu_id: i64,               //菜单ID
    pub create_time: NaiveDateTime, //创建时间
}
