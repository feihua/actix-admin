// author：刘飞华
// createTime：2024/12/17 09:08:59

use serde::{Deserialize, Serialize};

/*
添加角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoleReq {
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

/*
删除角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRoleReq {
    pub ids: Vec<i64>,
}

/*
更新角色信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleReq {
    pub id: i64,                //主键
    pub role_name: String,      //名称
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

/*
更新角色信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleStatusReq {
    pub ids: Vec<i64>,
    pub status: i64,
}

/*
查询角色信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleDetailReq {
    pub id: i64,
}

/*
查询角色信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleDetailResp {
    pub id: i64,             //主键
    pub role_name: String,   //名称
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryRoleDetailResp {
    pub fn new() -> QueryRoleDetailResp {
        QueryRoleDetailResp {
            id: 0,                       //主键
            role_name: "".to_string(),   //名称
            status_id: 0,                //状态(1:正常，0:禁用)
            sort: 0,                     //排序
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
}

/*
查询角色信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>, //名称
    pub status_id: Option<i8>,     //状态(1:正常，0:禁用)
}

/*
查询角色信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleListDataResp {
    pub id: i64,             //主键
    pub role_name: String,   //名称
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl RoleListDataResp {
    pub fn new() -> Vec<RoleListDataResp> {
        Vec::new()
    }
}

/*
查询角色菜单请求参数
*/
#[derive(Debug, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i64,
}

/*
角色菜单列表响应参数
*/
#[derive(Debug, Serialize)]
pub struct QueryRoleMenuData {
    pub menu_ids: Vec<i64>,
    pub menu_list: Vec<MenuList>,
}

/*
菜单信息
*/
#[derive(Debug, Serialize)]
pub struct MenuList {
    pub id: i64,
    pub parent_id: i64,
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

/*
更新角色菜单
*/
#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>, //菜单ids
    pub role_id: i64,       //角色id
}
