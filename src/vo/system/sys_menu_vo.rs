// author：刘飞华
// createTime：2024/12/19 14:21:03

use serde::{Deserialize, Serialize};

/*
添加菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddMenuReq {
    pub menu_name: String,         //菜单名称
    pub menu_type: i8,             //菜单类型(1：目录   2：菜单   3：按钮)
    pub status: i8,                //状态(1:正常，0:禁用)
    pub sort: i32,                 //排序
    pub parent_id: i64,            //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>,    //备注
}

/*
删除菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteMenuReq {
    pub id: i64, //菜单Id
}

/*
更新菜单信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuReq {
    pub id: i64,                   //主键
    pub menu_name: String,         //菜单名称
    pub menu_type: i8,             //菜单类型(1：目录   2：菜单   3：按钮)
    pub status: i8,                //状态(1:正常，0:禁用)
    pub sort: i32,                 //排序
    pub parent_id: i64,            //父ID
    pub menu_url: Option<String>,  //路由路径
    pub api_url: Option<String>,   //接口URL
    pub menu_icon: Option<String>, //菜单图标
    pub remark: Option<String>,    //备注
}

/*
更新菜单信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMenuStatusReq {
    pub ids: Vec<i64>, //菜单Ids
    pub status: i8,    //菜单状态
}

/*
查询菜单信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMenuDetailReq {
    pub id: i64, //菜单Id
}

/*
查询菜单信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMenuDetailResp {
    pub id: i64,             //主键
    pub menu_name: String,   //菜单名称
    pub menu_type: i8,       //菜单类型(1：目录   2：菜单   3：按钮)
    pub status: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub parent_id: i64,      //父ID
    pub menu_url: String,    //路由路径
    pub api_url: String,     //接口URL
    pub menu_icon: String,   //菜单图标
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryMenuDetailResp {
    pub fn new() -> QueryMenuDetailResp {
        QueryMenuDetailResp {
            id: 0,                       //主键
            menu_name: "".to_string(),   //菜单名称
            menu_type: 0,                //菜单类型(1：目录   2：菜单   3：按钮)
            status: 0,                   //状态(1:正常，0:禁用)
            sort: 0,                     //排序
            parent_id: 0,                //父ID
            menu_url: "".to_string(),    //路由路径
            api_url: "".to_string(),     //接口URL
            menu_icon: "".to_string(),   //菜单图标
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
}

/*
查询菜单信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMenuListReq {}

/*
查询菜单信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MenuListDataResp {
    pub id: i64,             //主键
    pub menu_name: String,   //菜单名称
    pub menu_type: i8,       //菜单类型(1：目录   2：菜单   3：按钮)
    pub status: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub parent_id: i64,      //父ID
    pub menu_url: String,    //路由路径
    pub api_url: String,     //接口URL
    pub menu_icon: String,   //菜单图标
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl MenuListDataResp {
    pub fn new() -> Vec<MenuListDataResp> {
        Vec::new()
    }
}
