use crate::AppState;
use actix_web::http::header;
use actix_web::{get, post, web, HttpRequest, Responder, Result};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::to_value;
use std::collections::{HashMap, HashSet};

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::User;
use crate::model::system::sys_user_role_model::UserRole;
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::system::sys_user_vo::*;

/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/addUser")]
pub async fn add_sys_user(
    item: web::Json<AddUserReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("add sys_user params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let sys_user = User {
        id: None,                       //主键
        mobile: req.mobile,             //手机
        user_name: req.user_name,       //姓名
        password: "123456".to_string(), //默认密码为123456,暂时不加密
        status_id: req.status_id,       //状态(1:正常，0:禁用)
        sort: req.sort,                 //排序
        remark: req.remark,             //备注
        create_time: None,              //创建时间
        update_time: None,              //修改时间
    };

    let result = User::insert(rb, &sys_user).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/deleteUser")]
pub async fn delete_sys_user(
    item: web::Json<DeleteUserReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("delete sys_user params: {:?}", &item);
    let rb = &data.batis;

    let ids = item.ids.clone();
    //id为1的用户为系统预留用户,不能删除
    if ids.contains(&1) {
        return BaseResponse::<String>::err_result_msg("系统预留用户,不能删除".to_string());
    }

    let result = User::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/updateUser")]
pub async fn update_sys_user(
    item: web::Json<UpdateUserReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_user params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let result = User::select_by_id(rb, req.id.clone()).await.unwrap();
    match result {
        None => BaseResponse::<String>::err_result_msg("用户不存在".to_string()),
        Some(s_user) => {
            let sys_user = User {
                id: Some(req.id),          //主键
                mobile: req.mobile,        //手机
                user_name: req.user_name,  //姓名
                password: s_user.password, //密码
                status_id: req.status_id,  //状态(1:正常，0:禁用)
                sort: req.sort,            //排序
                remark: req.remark,        //备注
                create_time: None,         //创建时间
                update_time: None,         //修改时间
            };

            let result = User::update_by_column(rb, &sys_user, "id").await;

            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
    }
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/updateUserStatus")]
pub async fn update_sys_user_status(
    item: web::Json<UpdateUserStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_user_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let param = vec![to_value!(req.status), to_value!(req.ids)];
    let result = rb
        .exec("update sys_user set status = ? where id in ?", param)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新用户密码
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/update_user_password")]
pub async fn update_user_password(
    item: web::Json<UpdateUserPwdReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;

    let rb = &data.batis;

    let sys_user_result = User::select_by_id(rb, user_pwd.id).await;

    match sys_user_result {
        Ok(user_result) => match user_result {
            None => BaseResponse::<String>::err_result_msg("用户不存在".to_string()),
            Some(mut user) => {
                if user.password == user_pwd.pwd {
                    user.password = user_pwd.re_pwd;
                    let result = User::update_by_column(rb, &user, "id").await;

                    match result {
                        Ok(_u) => BaseResponse::<String>::ok_result(),
                        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                    }
                } else {
                    BaseResponse::<String>::err_result_msg("旧密码不正确".to_string())
                }
            }
        },
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/queryUserDetail")]
pub async fn query_sys_user_detail(
    item: web::Json<QueryUserDetailReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_user_detail params: {:?}", &item);
    let rb = &data.batis;

    let result = User::select_by_id(rb, item.id).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_user = QueryUserDetailResp {
                id: x.id.unwrap(),                                 //主键
                mobile: x.mobile,                                  //手机
                user_name: x.user_name,                            //姓名
                status_id: x.status_id,                            //状态(1:正常，0:禁用)
                sort: x.sort,                                      //排序
                remark: x.remark.unwrap_or_default(),              //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
            };

            BaseResponse::<QueryUserDetailResp>::ok_result_data(sys_user)
        }
        Err(err) => BaseResponse::<QueryUserDetailResp>::err_result_data(
            QueryUserDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/queryUserList")]
pub async fn query_sys_user_list(
    item: web::Json<QueryUserListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_user_list params: {:?}", &item);
    let rb = &data.batis;

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.unwrap_or_default();

    let page = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = User::select_page_by_name(rb, page, mobile, status_id).await;

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();

    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                let sys_user = UserListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    mobile: x.mobile,                                  //手机
                    user_name: x.user_name,                            //姓名
                    status_id: x.status_id,                            //状态(1:正常，0:禁用)
                    sort: x.sort,                                      //排序
                    remark: x.remark.unwrap_or_default(),              //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                };
                sys_user_list_data.push(sys_user);
            }

            BaseResponse::<Vec<UserListDataResp>>::ok_result_page(sys_user_list_data, total)
        }
        Err(err) => BaseResponse::<Vec<UserListDataResp>>::err_result_page(
            UserListDataResp::new(),
            err.to_string(),
        ),
    }
}
/*
 *后台用户登录
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/login")]
pub async fn login(
    item: web::Json<UserLoginReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("user login params: {:?}, {:?}", &item, data.batis);
    let rb = &data.batis;

    let user_result = User::select_by_mobile(rb, &item.mobile).await;
    log::info!("select_by_mobile: {:?}", user_result);

    match user_result {
        Ok(u) => match u {
            None => {
                return BaseResponse::<String>::err_result_msg("用户不存在".to_string());
            }
            Some(user) => {
                let id = user.id.unwrap();
                let username = user.user_name;
                let password = user.password;

                if password.ne(&item.password) {
                    return BaseResponse::<String>::err_result_msg("密码不正确".to_string());
                }

                let btn_menu = query_btn_menu(&id, data).await;

                if btn_menu.len() == 0 {
                    return BaseResponse::<String>::err_result_msg(
                        "用户没有分配角色或者菜单,不能登录".to_string(),
                    );
                }

                match JWTToken::new(id, &username, btn_menu).create_token("123") {
                    Ok(token) => BaseResponse::<String>::ok_result_data(token),
                    Err(err) => {
                        let er = match err {
                            WhoUnfollowedError::JwtTokenError(s) => s,
                            _ => "no math error".to_string(),
                        };

                        BaseResponse::<String>::err_result_msg(er)
                    }
                }
            }
        },

        Err(err) => {
            log::info!("select_by_column: {:?}", err);
            BaseResponse::<String>::err_result_msg("查询用户异常".to_string())
        }
    }
}

/*
 *查询用户按钮权限
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
async fn query_btn_menu(id: &i64, data: web::Data<AppState>) -> Vec<String> {
    let rb = &data.batis;
    let user_role = UserRole::is_admin(rb, id).await;
    let mut btn_menu: Vec<String> = Vec::new();
    if user_role.unwrap().len() == 1 {
        let data = Menu::select_all(rb).await;

        for x in data.unwrap() {
            btn_menu.push(x.api_url.unwrap_or_default());
        }
        log::info!("admin login: {:?}", id);
        btn_menu
    } else {
        let btn_menu_map: Vec<HashMap<String, String>> = rb.query_decode("select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?", vec![to_value!(id)]).await.unwrap();
        for x in btn_menu_map {
            btn_menu.push(x.get("api_url").unwrap().to_string());
        }
        log::info!("ordinary login: {:?}", id);
        btn_menu
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/query_user_role")]
pub async fn query_user_role(
    item: web::Json<QueryUserRoleReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query_user_role params: {:?}", item);
    let rb = &data.batis;

    let user_role = UserRole::select_by_column(rb, "user_id", &item.user_id).await;
    let mut user_role_ids: Vec<i64> = Vec::new();

    for x in user_role.unwrap() {
        user_role_ids.push(x.role_id);
    }

    let sys_role = Role::select_all(rb).await;

    let mut sys_role_list: Vec<RoleList> = Vec::new();

    for x in sys_role.unwrap() {
        sys_role_list.push(RoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });
    }

    BaseResponse::<QueryUserRoleResp>::ok_result_data(QueryUserRoleResp {
        sys_role_list,
        user_role_ids,
    })
}

/*
 *更新用户角色
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/update_user_role")]
pub async fn update_user_role(
    item: web::Json<UpdateUserRoleReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update_user_role params: {:?}", item);
    let rb = &data.batis;

    let user_role = item.0;
    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id == 1 {
        return BaseResponse::<String>::err_result_msg("不能修改超级管理员的角色".to_string());
    }

    let sys_result = UserRole::delete_by_column(rb, "user_id", user_id).await;

    if sys_result.is_err() {
        return BaseResponse::<String>::err_result_msg("更新用户角色异常".to_string());
    }

    let mut sys_role_user_list: Vec<UserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = UserRole::insert_batch(rb, &sys_role_user_list, len as u64).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

#[get("/query_user_menu")]
pub async fn query_user_menu(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let def = header::HeaderValue::from_str("").unwrap();
    let token = req
        .headers()
        .get("Authorization")
        .unwrap_or(&def)
        .to_str()
        .ok()
        .unwrap();

    let split_vec = token.split_whitespace().collect::<Vec<_>>();
    if split_vec.len() != 2 || split_vec[0] != "Bearer" {
        return BaseResponse::<QueryUserMenuResp>::err_result_data(
            QueryUserMenuResp::new(),
            "the token format wrong".to_string(),
        );
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => data,
        Err(err) => {
            return BaseResponse::<QueryUserMenuResp>::err_result_data(
                QueryUserMenuResp::new(),
                err.to_string(),
            )
        }
    };

    log::info!("query user menu params {:?}", jwt_token);

    let rb = &data.batis;

    //根据id查询用户
    let result = User::select_by_id(rb, jwt_token.id).await;

    let mut sys_menu: Vec<MenuList> = Vec::new();
    let mut btn_menu: Vec<String> = Vec::new();

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => BaseResponse::<QueryUserMenuResp>::err_result_data(
                    QueryUserMenuResp::new(),
                    "用户不存在".to_string(),
                ),
                Some(user) => {
                    //role_id为1是超级管理员--判断是不是超级管理员
                    let sql_str =
                        "select count(id) from sys_user_role where role_id = 1 and user_id = ?";
                    let count = rb
                        .query_decode::<i32>(sql_str, vec![to_value!(user.id)])
                        .await
                        .unwrap();

                    let sys_menu_list: Vec<Menu>;

                    if count > 0 {
                        sys_menu_list = Menu::select_all(rb).await.unwrap_or_default();
                    } else {
                        let sql_str = "select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ?";
                        sys_menu_list = rb
                            .query_decode(sql_str, vec![to_value!(user.id)])
                            .await
                            .unwrap();
                    }

                    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.id.unwrap_or_default().clone());
                            sys_menu_ids.insert(x.parent_id.clone());
                        }

                        if x.api_url.clone().unwrap_or_default().len() > 0 {
                            btn_menu.push(x.api_url.unwrap_or_default());
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for id in sys_menu_ids {
                        menu_ids.push(id)
                    }
                    let menu_result = Menu::select_by_ids(rb, &menu_ids).await.unwrap();
                    for menu in menu_result {
                        sys_menu.push(MenuList {
                            id: menu.id.unwrap(),
                            parent_id: menu.parent_id,
                            name: menu.menu_name,
                            icon: menu.menu_icon.unwrap_or_default(),
                            api_url: menu.api_url.as_ref().unwrap().to_string(),
                            menu_type: menu.menu_type,
                            path: menu.menu_url.unwrap_or_default(),
                        });
                    }

                    let resp = QueryUserMenuResp {
                        sys_menu,
                        btn_menu,
                        avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                        name: user.user_name,
                    };

                    BaseResponse::<QueryUserMenuResp>::ok_result_data(resp)
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => BaseResponse::<QueryUserMenuResp>::err_result_data(
            QueryUserMenuResp::new(),
            err.to_string(),
        ),
    }
}
