use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct AddSysMenu {
    pub menu_name: String,          //菜单名称
    pub menu_type: i8,              //菜单类型(1：目录   2：菜单   3：按钮)
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub parent_id: i64,             //父ID
    pub menu_url: Option<String>,   //路由路径
    pub api_url: Option<String>,    //接口URL
    pub menu_icon: Option<String>,  //菜单图标
    pub remark: Option<String>,     //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}

#[derive(Debug, PartialEq, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UpdateSysMenu {
    pub id: i64,                    //主键
    pub menu_name: String,          //菜单名称
    pub menu_type: i8,              //菜单类型(1：目录   2：菜单   3：按钮)
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub parent_id: i64,             //父ID
    pub menu_url: Option<String>,   //路由路径
    pub api_url: Option<String>,    //接口URL
    pub menu_icon: Option<String>,  //菜单图标
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
#[diesel(table_name = crate::schema::sys_menu)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct SysMenu {
    pub id: i64,                    //主键
    pub menu_name: String,          //菜单名称
    pub menu_type: i8,              //菜单类型(1：目录   2：菜单   3：按钮)
    pub status_id: i8,              //状态(1:正常，0:禁用)
    pub sort: i32,                  //排序
    pub parent_id: i64,             //父ID
    pub menu_url: String,           //路由路径
    pub api_url: String,            //接口URL
    pub menu_icon: String,          //菜单图标
    pub remark: Option<String>,     //备注
    pub create_time: NaiveDateTime, //创建时间
    pub update_time: NaiveDateTime, //修改时间
}
#[derive(QueryableByName)]
pub struct StringColumn {
    #[diesel(sql_type = diesel::sql_types::Varchar)]
    pub api_url: String,
}
