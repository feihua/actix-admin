use std::collections::HashMap;
use actix_web::{HttpRequest, post, get, Responder, Result, web};
use actix_web::http::header;
use rbatis::rbdc::datetime::DateTime;
use rbatis::sql::{PageRequest};
use rbs::to_value;
use crate::AppState;
use crate::model::user::{SysUser};
use crate::model::menu::{SysMenu};
use crate::model::role::{SysRole};
use crate::model::user_role::{SysUserRole};
use crate::utils::error::WhoUnfollowedError;
use crate::vo::user_vo::*;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{BaseResponse, handle_result};

// 后台用户登录
#[post("/login")]
pub async fn login(item: web::Json<UserLoginReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user login params: {:?}, {:?}", &item, data.batis);
    let mut rb = &data.batis;

    let user_result = SysUser::select_by_column(&mut rb, "mobile", &item.mobile).await;
    log::info!("select_by_column: {:?}",user_result);

    match user_result {
        Ok(d) => {
            if d.len() == 0 {
                let resp = BaseResponse {
                    msg: "用户不存在".to_string(),
                    code: 1,
                    data: None,
                };
                return Ok(web::Json(resp));
            }

            let user = d.get(0).unwrap().clone();
            let id = user.id.unwrap();
            let username = user.user_name;
            let password = user.password;

            if password.ne(&item.password) {
                let resp = BaseResponse {
                    msg: "密码不正确".to_string(),
                    code: 1,
                    data: None,
                };
                return Ok(web::Json(resp));
            }

            let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

            let mut btn_menu: Vec<String> = Vec::new();

            for x in data.unwrap().records {
                btn_menu.push(x.api_url.unwrap_or_default());
            }

            match JWTToken::new(id, &username, btn_menu).create_token("123") {
                Ok(token) => {
                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(UserLoginData {
                            mobile: item.mobile.to_string(),
                            token,
                        }),
                    };

                    Ok(web::Json(resp))
                }
                Err(err) => {
                    let er = match err {
                        WhoUnfollowedError::JwtTokenError(s) => { s }
                        _ => "no math error".to_string()
                    };
                    let resp = BaseResponse {
                        msg: er,
                        code: 1,
                        data: None,
                    };

                    Ok(web::Json(resp))
                }
            }
        }

        Err(err) => {
            log::info!("select_by_column: {:?}",err);
            let resp = BaseResponse {
                msg: "查询用户异常".to_string(),
                code: 1,
                data: None,
            };
            return Ok(web::Json(resp));
        }
    }
}

#[post("/query_user_role")]
pub async fn query_user_role(item: web::Json<QueryUserRoleReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query_user_role params: {:?}", item);
    let mut rb = &data.batis;

    let sys_role = SysRole::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();
    let mut user_role_ids: Vec<i32> = Vec::new();

    for x in sys_role.unwrap().records {
        sys_role_list.push(UserRoleList {
            id: x.id.unwrap(),
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark.unwrap_or_default(),
            create_time: x.create_time.unwrap().0.to_string(),
            update_time: x.update_time.unwrap().0.to_string(),
        });

        user_role_ids.push(x.id.unwrap_or_default());
    }

    let resp = QueryUserRoleResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryUserRoleData {
            sys_role_list,
            user_role_ids,
        },
    };

    Ok(web::Json(resp))
}

#[post("/update_user_role")]
pub async fn update_user_role(item: web::Json<UpdateUserRoleReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update_user_role params: {:?}", item);
    let mut rb = &data.batis;

    let user_role = item.0;
    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;
    let len = user_role.role_ids.len();

    if user_id == 1 {
        let resp = BaseResponse {
            msg: "不能修改超级管理员的角色".to_string(),
            code: 1,
            data: None,
        };
        return Ok(web::Json(resp));
    }

    let sys_result = SysUserRole::delete_by_column(&mut rb, "user_id", user_id).await;

    if sys_result.is_err() {
        let resp = BaseResponse {
            msg: "更新用户角色异常".to_string(),
            code: 1,
            data: None,
        };
        return Ok(web::Json(resp));
    }

    let mut sys_role_user_list: Vec<SysUserRole> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        sys_role_user_list.push(SysUserRole {
            id: None,
            create_time: Some(DateTime::now()),
            update_time: Some(DateTime::now()),
            status_id: 1,
            sort: 1,
            role_id: r_id,
            user_id: user_id.clone(),
        })
    }

    let result = SysUserRole::insert_batch(&mut rb, &sys_role_user_list, len as u64).await;

    return Ok(web::Json(handle_result(result)));
}


#[get("/query_user_menu")]
pub async fn query_user_menu(req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder> {
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
        let resp = BaseResponse {
            msg: "the token format wrong".to_string(),
            code: 1,
            data: None,
        };
        return Ok(web::Json(resp));
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => { data }
        Err(err) => {
            let resp = BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            };
            return Ok(web::Json(resp));
        }
    };

    log::info!("query user menu params {:?}",jwt_token);

    let mut rb = &data.batis;

    //根据id查询用户
    let result = SysUser::select_by_id(&mut rb, jwt_token.id).await;

    match result {
        Ok(sys_user) => {
            match sys_user {
                // 用户不存在的情况
                None => {
                    Ok(web::Json(BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: None,
                    }))
                }
                Some(user) => {
                    let user_role = SysUserRole::select_by_column(&mut rb, "user_id", user.id).await;
                    // 判断是不是超级管理员
                    let mut is_admin = false;

                    for x in user_role.unwrap() {
                        if x.role_id == 1 {
                            is_admin = true;
                            break;
                        }
                    }

                    let sys_menu_list: Vec<SysMenu>;

                    if is_admin {
                        sys_menu_list = SysMenu::select_all(&mut rb).await.unwrap_or_default();
                    } else {
                        sys_menu_list = rb.query_decode("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc", vec![to_value!(user.id)]).await.unwrap();
                    }

                    let mut sys_menu_map: HashMap<i32, MenuUserList> = HashMap::new();
                    let mut sys_menu: Vec<MenuUserList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_parent_ids: Vec<i32> = Vec::new();

                    for x in sys_menu_list {
                        let y = x.clone();
                        if y.menu_type != 3 {
                            sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                id: y.id.unwrap(),
                                parent_id: y.parent_id,
                                name: y.menu_name,
                                icon: y.menu_icon.unwrap_or_default(),
                                api_url: y.api_url.as_ref().unwrap().to_string(),
                                menu_type: y.menu_type,
                                path: y.menu_url.unwrap_or_default(),
                            });
                            sys_menu_parent_ids.push(y.parent_id.clone())
                        }

                        btn_menu.push(x.api_url.unwrap_or_default());
                    }

                    for menu_id in sys_menu_parent_ids {
                        let s_menu_result = SysMenu::select_by_id(&mut rb, menu_id).await.unwrap();
                        match s_menu_result {
                            None => {}
                            Some(y) => {
                                sys_menu_map.insert(y.id.unwrap(), MenuUserList {
                                    id: y.id.unwrap(),
                                    parent_id: y.parent_id,
                                    name: y.menu_name,
                                    icon: y.menu_icon.unwrap_or_default(),
                                    api_url: y.api_url.as_ref().unwrap().to_string(),
                                    menu_type: y.menu_type,
                                    path: y.menu_url.unwrap_or_default(),
                                });
                            }
                        }
                    }

                    let mut sys_menu_ids: Vec<i32> = Vec::new();
                    for menu in &sys_menu_map {
                        sys_menu_ids.push(menu.0.abs())
                    }

                    sys_menu_ids.sort();

                    for id in sys_menu_ids {
                        let menu = sys_menu_map.get(&id).cloned().unwrap();
                        sys_menu.push(menu)
                    }

                    let resp = BaseResponse {
                        msg: "successful".to_string(),
                        code: 0,
                        data: Some(QueryUserMenuData {
                            sys_menu,
                            btn_menu,
                            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                            name: user.user_name,
                        }),
                    };
                    Ok(web::Json(resp))
                }
            }
        }
        // 查询用户数据库异常
        Err(err) => {
            Ok(web::Json(BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            }))
        }
    }
}

// 查询用户列表
#[post("/user_list")]
pub async fn user_list(item: web::Json<UserListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query user_list params: {:?}", &item);
    let mut rb = &data.batis;

    let mobile = item.mobile.as_deref().unwrap_or_default();
    let status_id = item.status_id.as_deref().unwrap_or_default();

    let page_req = &PageRequest::new(item.page_no.clone(), item.page_size.clone());
    let result = SysUser::select_page_by_name(&mut rb, page_req, mobile, status_id).await;

    let resp = match result {
        Ok(page) => {
            let total = page.total;

            let mut list_data: Vec<UserListData> = Vec::new();

            for user in page.records {
                list_data.push(UserListData {
                    id: user.id.unwrap(),
                    sort: user.sort,
                    status_id: user.status_id,
                    mobile: user.mobile,
                    user_name: user.user_name,
                    remark: user.remark.unwrap_or_default(),
                    create_time: user.create_time.unwrap().0.to_string(),
                    update_time: user.update_time.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                success: true,
                total,
                data: Some(list_data),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    Ok(web::Json(resp))
}

// 添加用户信息
#[post("/user_save")]
pub async fn user_save(item: web::Json<UserSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;

    let mut rb = &data.batis;
    let sys_user = SysUser {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: user.status_id,
        sort: user.sort,
        mobile: user.mobile,
        user_name: user.user_name,
        remark: user.remark,
        password: "123456".to_string(),//默认密码为123456,暂时不加密
    };

    let result = SysUser::insert(&mut rb, &sys_user).await;

    Ok(web::Json(handle_result(result)))
}

// 更新用户信息
#[post("/user_update")]
pub async fn user_update(item: web::Json<UserUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;
    let mut rb = &data.batis;

    let result = SysUser::select_by_id(&mut rb, user.id.clone()).await.unwrap();

    match result {
        None => {
            Ok(web::Json(BaseResponse {
                msg: "用户不存在".to_string(),
                code: 1,
                data: Some("None".to_string()),
            }))
        }
        Some(s_user) => {
            let sys_user = SysUser {
                id: Some(user.id),
                create_time: s_user.create_time,
                update_time: Some(DateTime::now()),
                status_id: user.status_id,
                sort: user.sort,
                mobile: user.mobile,
                user_name: user.user_name,
                remark: user.remark,
                password: s_user.password,
            };

            let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

            Ok(web::Json(handle_result(result)))
        }
    }
}

// 删除用户信息
#[post("/user_delete")]
pub async fn user_delete(item: web::Json<UserDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_delete params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysUser::delete_in_column(&mut rb, "id", &item.ids).await;

    Ok(web::Json(handle_result(result)))
}

// 更新用户密码
#[post("/update_user_password")]
pub async fn update_user_password(item: web::Json<UpdateUserPwdReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;

    let mut rb = &data.batis;

    let sys_user_result = SysUser::select_by_id(&mut rb, user_pwd.id).await;

    match sys_user_result {
        Ok(user_result) => {
            match user_result {
                None => {
                    let resp = BaseResponse {
                        msg: "用户不存在".to_string(),
                        code: 1,
                        data: None,
                    };
                    Ok(web::Json(resp))
                }
                Some(mut user) => {
                    if user.password == user_pwd.pwd {
                        user.password = user_pwd.re_pwd;
                        let result = SysUser::update_by_column(&mut rb, &user, "id").await;

                        Ok(web::Json(handle_result(result)))
                    } else {
                        let resp = BaseResponse {
                            msg: "旧密码不正确".to_string(),
                            code: 1,
                            data: None,
                        };
                        Ok(web::Json(resp))
                    }
                }
            }
        }
        Err(err) => {
            let resp = BaseResponse {
                msg: err.to_string(),
                code: 1,
                data: None,
            };
            Ok(web::Json(resp))
        }
    }
}