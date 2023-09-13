use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuListReq {
    pub menu_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuListData {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub parent_id: i64,
    pub menu_name: String,
    pub label: String,
    pub menu_url: String,
    pub icon: String,
    pub api_url: String,
    pub remark: String,
    pub menu_type: i8,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuSaveReq {
    pub sort: i32,
    pub status_id: i8,
    pub parent_id: Option<i64>,
    pub menu_name: String,
    pub menu_url: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUpdateReq {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub parent_id: i64,
    pub menu_name: String,
    pub menu_url: Option<String>,
    pub icon: Option<String>,
    pub api_url: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuDeleteReq {
    pub id: i64,
}
