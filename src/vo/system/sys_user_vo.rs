// author：刘飞华
// createTime：2024/12/19 14:21:03

use serde::{Deserialize, Serialize};

/*
添加用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserReq {
    pub mobile: String,           //手机
    pub user_name: String,        //姓名
    pub password: Option<String>, //密码
    pub status_id: i8,            //状态(1:正常，0:禁用)
    pub sort: i32,                //排序
    pub remark: Option<String>,   //备注
}

/*
删除用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserReq {
    pub ids: Vec<i64>, //用户ids
}

/*
更新用户信息请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub id: i64,                //主键
    pub mobile: String,         //手机
    pub user_name: String,      //姓名
    pub status_id: i8,          //状态(1:正常，0:禁用)
    pub sort: i32,              //排序
    pub remark: Option<String>, //备注
}

/*
更新用户信息状态请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserStatusReq {
    pub ids: Vec<i64>, //用户ids
    pub status: i8,    //状态
}

/*
查询用户信息详情请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserDetailReq {
    pub id: i64, //用户id
}

/*
查询用户信息详情响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserDetailResp {
    pub id: i64,             //主键
    pub mobile: String,      //手机
    pub user_name: String,   //姓名
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

impl QueryUserDetailResp {
    pub fn new() -> QueryUserDetailResp {
        QueryUserDetailResp {
            id: 0,                       //主键
            mobile: "".to_string(),      //手机
            user_name: "".to_string(),   //姓名
            status_id: 0,                //状态(1:正常，0:禁用)
            sort: 0,                     //排序
            remark: "".to_string(),      //备注
            create_time: "".to_string(), //创建时间
            update_time: "".to_string(), //修改时间
        }
    }
}

/*
查询用户信息列表请求参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub mobile: Option<String>,    //手机
    pub user_name: Option<String>, //姓名
    pub status_id: Option<i8>,     //状态(1:正常，0:禁用)
}

/*
查询用户信息列表响应参数
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListDataResp {
    pub id: i64,             //主键
    pub mobile: String,      //手机
    pub user_name: String,   //姓名
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}
impl UserListDataResp {
    pub fn new() -> Vec<UserListDataResp> {
        Vec::new()
    }
}

/*
用户登录请求参数
*/
#[derive(Debug, Deserialize)]
pub struct UserLoginReq {
    pub mobile: String,   //手机号码
    pub password: String, //密码
}

/*
查询用户角色请求参数
*/
#[derive(Debug, Deserialize)]
pub struct QueryUserRoleReq {
    pub user_id: i64, //用户id
}

/*
查询用户角色列表响应参数
*/
#[derive(Debug, Serialize)]
pub struct QueryUserRoleResp {
    pub role_list: Vec<RoleList>, //角色列表
    pub role_ids: Vec<i64>,       //角色ids
}

/*
角色信息
*/
#[derive(Debug, Serialize)]
pub struct RoleList {
    pub id: i64,             //主键
    pub role_name: String,   //名称
    pub status_id: i8,       //状态(1:正常，0:禁用)
    pub sort: i32,           //排序
    pub remark: String,      //备注
    pub create_time: String, //创建时间
    pub update_time: String, //修改时间
}

/*
更新用户角色请求参数
*/
#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleReq {
    pub user_id: i64,       //用户主键
    pub role_ids: Vec<i64>, //角色主键
}

/*
查询用户菜单请求参数
*/
#[derive(Debug, Deserialize)]
pub struct QueryUserMenuReq {
    pub token: String,
}

/*
查询用户菜单列表响应参数
*/
#[derive(Debug, Serialize)]
pub struct QueryUserMenuResp {
    pub sys_menu: Vec<MenuList>, //菜单列表
    pub btn_menu: Vec<String>,   //菜单按钮
    pub avatar: String,          //头像
    pub name: String,            //名称
}

#[derive(Debug, Serialize, Clone)]
pub struct MenuList {
    pub id: i64,         //主键
    pub parent_id: i64,  //父ID
    pub name: String,    //菜单名称
    pub path: String,    //路由路径
    pub api_url: String, //接口URL
    pub menu_type: i8,   //菜单类型(1：目录   2：菜单   3：按钮)
    pub icon: String,    //菜单图标
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPwdReq {
    pub id: i64,        //用户主键
    pub pwd: String,    //用户密码
    pub re_pwd: String, //用户密码
}
