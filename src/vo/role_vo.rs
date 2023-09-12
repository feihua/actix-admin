use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status_id: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListData {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleSaveReq {
    pub role_name: String,
    pub sort: i32,
    pub status_id: i8,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleUpdateReq {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub role_name: String,
    pub remark: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDeleteReq {
    pub ids: Vec<i64>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i64>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDataList {
    pub id: i64,
    pub parent_id: i64,
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>,
    pub role_id: i64,
}


