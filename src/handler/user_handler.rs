use actix_web::{post, Responder, Result, web};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use redis::Commands;
use crate::AppState;
use crate::model::entity::{SysMenu, SysUser};
use crate::utils::error::WhoUnfollowedError;
use crate::vo::user_vo::*;
use crate::utils::jwt_util::JWTToken;
use crate::utils::redis_util::init_redis;
use crate::vo::{BaseResponse, handle_result};

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
            let id = user.id.unwrap().to_string();
            let username = user.real_name.unwrap();
            let password = user.password.unwrap();

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

            match JWTToken::new(&id, &username, btn_menu).create_token("123") {
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


#[post("/query_user_menu")]
pub async fn query_user_menu(item: web::Json<QueryUserMenuReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query_user_menu params: {:?}", &item);
    let mut rb = &data.batis;

    let sys_user = SysUser::select_by_column(&mut rb, "id", "1").await;

    let data = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let mut sys_menu: Vec<MenuUserList> = Vec::new();
    let mut btn_menu: Vec<String> = Vec::new();
    let mut btn_menu_str: String = String::new();

    for x in data.unwrap().records {
        let y = x.clone();
        sys_menu.push(MenuUserList {
            id: y.id.unwrap(),
            parent_id: y.parent_id.unwrap(),
            name: y.menu_name.unwrap_or_default(),
            icon: y.menu_icon.unwrap_or_default(),
            api_url: y.api_url.as_ref().unwrap().to_string(),
            menu_type: y.menu_type.unwrap(),
            path: y.api_url.unwrap_or_default(),
        });

        btn_menu.push(x.api_url.unwrap_or_default());
        btn_menu_str.push_str(&x.menu_name.unwrap_or_default());
        btn_menu_str.push_str(&",")
    }

    let mut redis_con = init_redis().await;
    let _: () = redis_con.set("btn_menu_str", btn_menu_str).expect("sdfs");

    let resp = BaseResponse {
        msg: "successful".to_string(),
        code: 0,
        data: Some(QueryUserMenuData {
            sys_menu,
            btn_menu,
            avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
            name: sys_user.unwrap_or_default().get(0).unwrap().real_name.as_ref().expect("用户名不存在").to_string(),
        }),
    };
    Ok(web::Json(resp))
}


#[post("/user_list")]
pub async fn user_list(item: web::Json<UserListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query user_list params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysUser::select_page(&mut rb, &PageRequest::new(item.page_no, item.page_size)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;
            let page_no = d.page_no;
            let page_size = d.page_size;

            let mut user_list: Vec<UserListData> = Vec::new();

            for x in d.records {
                user_list.push(UserListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    mobile: x.mobile.unwrap_or_default(),
                    real_name: x.real_name.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }

            UserListResp {
                msg: "successful".to_string(),
                code: 0,
                page_no,
                page_size,
                success: true,
                total,
                data: Some(user_list),
            }
        }
        Err(err) => {
            UserListResp {
                msg: err.to_string(),
                code: 1,
                page_no: 0,
                page_size: 0,
                success: true,
                total: 0,
                data: None,
            }
        }
    };

    Ok(web::Json(resp))
}


#[post("/user_save")]
pub async fn user_save(item: web::Json<UserSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;

    let mut rb = &data.batis;
    let sys_user = SysUser {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: None,
        status_id: Some(1),
        sort: Some(1),
        user_no: Some(1),
        mobile: Some(user.mobile),
        real_name: Some(user.real_name),
        remark: Some(user.remark),
        password: Some("123456".to_string()),
    };

    let result = SysUser::insert(&mut rb, &sys_user).await;

    Ok(web::Json(handle_result(result)))
}


#[post("/user_update")]
pub async fn user_update(item: web::Json<UserUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;

    let mut rb = &data.batis;
    let sys_user = SysUser {
        id: Some(user.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(user.status_id),
        sort: Some(user.sort),
        user_no: None,
        mobile: Some(user.mobile),
        real_name: Some(user.real_name),
        remark: Some(user.remark),
        password: None,
    };

    let result = SysUser::update_by_column(&mut rb, &sys_user, "id").await;

    Ok(web::Json(handle_result(result)))
}


#[post("/user_delete")]
pub async fn user_delete(item: web::Json<UserDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_delete params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysUser::delete_in_column(&mut rb, "id", &item.ids).await;

    Ok(web::Json(handle_result(result)))
}

