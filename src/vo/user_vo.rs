use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginReq {
    pub mobile: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResp {
    pub msg: String,
    pub code: i32,
    pub status: String,
    pub data: Option<UserLoginData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginData {
    pub mobile: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserMenuReq {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserMenuResp {
    pub msg: String,
    pub code: i32,
    pub data: QueryUserMenuData,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserMenuData {
    pub sys_menu: Vec<MenuUserList>,
    pub btn_menu: Vec<String>,
    pub avatar: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuUserList {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub path: String,
    pub api_url: String,
    pub menu_type: i32,
    pub icon: String,
}


#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UserListReq {
    pub page_no: u64,
    pub page_size: u64,
    pub mobile: Option<String>,
    pub status_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResp {
    pub msg: String,
    pub code: i32,
    pub page_no: u64,
    pub page_size: u64,
    pub success: bool,
    pub total: u64,
    pub data: Option<Vec<UserListData>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListData {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub mobile: String,
    pub real_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSaveReq {
    pub mobile: String,
    pub real_name: String,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateReq {
    pub id: i32,
    pub sort: i32,
    pub status_id: i32,
    pub mobile: String,
    pub real_name: String,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDeleteReq {
    pub ids: Vec<i32>,
}
