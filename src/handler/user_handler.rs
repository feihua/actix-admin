use std::collections::HashSet;

use actix_web::{Either, get, HttpRequest, post, Responder, Result, web};
use actix_web::http::header;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder, QueryTrait, Statement};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::model::{sys_menu, sys_user, sys_user_role};
use crate::model::prelude::{SysMenu, SysRole, SysUser, SysUserRole};
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;
use crate::vo::{err_result_msg, ok_result_data, ok_result_msg, ok_result_page};
use crate::vo::user_vo::*;

// 后台用户登录
#[post("/login")]
pub async fn login(item: web::Json<UserLoginReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user login params: {:?}", &item);
    let conn = &data.conn;

    let user_result = SysUser::find().filter(sys_user::Column::Mobile.eq(&item.mobile)).one(conn).await.unwrap_or_default();
    log::info!("select_by_mobile: {:?}",user_result);

    if user_result.is_none() {
        return Ok(web::Json(err_result_msg("用户不存在!")));
    }

    let user = user_result.unwrap();

    let id = user.id;
    let username = user.user_name;
    let password = user.password;

    if password.ne(&item.password) {
        return Ok(web::Json(err_result_msg("密码不正确!")));
    }

    let btn_menu = query_btn_menu(conn, id.clone()).await?;

    if btn_menu.len() == 0 {
        return Ok(web::Json(err_result_msg("用户没有分配角色或者菜单,不能登录!")));
    }

    match JWTToken::new(id, &username, btn_menu).create_token("123") {
        Ok(token) => {
            Ok(web::Json(ok_result_data(token)))
        }
        Err(err) => {
            let er = match err {
                WhoUnfollowedError::JwtTokenError(s) => { s }
                _ => "no math error".to_string()
            };

            Ok(web::Json(err_result_msg(&er)))
        }
    }
}

// 登录的时候 查询权限
async fn query_btn_menu(conn: &DatabaseConnection, id: i64) -> Result<Vec<String>> {
    let mut btn_menu: Vec<String> = Vec::new();
    //角色Id为1的是系统预留超级管理员角色
    if SysUserRole::find().filter(sys_user_role::Column::UserId.eq(id.clone())).filter(sys_user_role::Column::RoleId.eq(1)).count(conn).await.unwrap_or_default() != 0 {
        for x in SysMenu::find().all(conn).await.unwrap_or_default() {
            btn_menu.push(x.api_url);
        }
        log::info!("admin login: {:?}",id);
    } else {
        let sql_str = r#"select distinct u.api_url from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = $1"#;
        for x in conn.query_all(Statement::from_sql_and_values(DatabaseBackend::MySql, sql_str, [id.into()])).await.unwrap_or_default() {
            btn_menu.push(x.try_get("", "api_url").unwrap_or_default());
        }
        log::info!("ordinary login: {:?}",id);
    }

    Ok(btn_menu)
}

#[post("/query_user_role")]
pub async fn query_user_role(item: web::Json<QueryUserRoleReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query_user_role params: {:?}", item);
    let conn = &data.conn;
    let mut user_role_ids: Vec<i64> = Vec::new();

    for x in SysUserRole::find().filter(sys_user_role::Column::UserId.eq(item.user_id.clone())).all(conn).await.unwrap_or_default() {
        user_role_ids.push(x.role_id);
    }

    let mut sys_role_list: Vec<UserRoleList> = Vec::new();

    for x in SysRole::find().all(conn).await.unwrap_or_default() {
        sys_role_list.push(UserRoleList {
            id: x.id,
            status_id: x.status_id,
            sort: x.sort,
            role_name: x.role_name,
            remark: x.remark,
            create_time: x.create_time.to_string(),
            update_time: x.update_time.to_string(),
        });
    }

    Ok(web::Json(ok_result_data(QueryUserRoleData { sys_role_list, user_role_ids })))
}

#[post("/update_user_role")]
pub async fn update_user_role(item: web::Json<UpdateUserRoleReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update_user_role params: {:?}", item);
    let conn = &data.conn;

    let user_role = item.0;
    let user_id = user_role.user_id;
    let role_ids = &user_role.role_ids;

    if user_id == 1 {
        return Ok(web::Json(err_result_msg("不能修改超级管理员的角色!")));
    }

    SysUserRole::delete_many().filter(sys_user_role::Column::UserId.eq(user_id)).exec(conn).await.unwrap();

    let mut sys_role_user_list: Vec<sys_user_role::ActiveModel> = Vec::new();
    for role_id in role_ids {
        let r_id = role_id.clone();
        if r_id == 1 {
            continue;
        }
        sys_role_user_list.push(sys_user_role::ActiveModel {
            id: NotSet,
            status_id: Set(1),
            sort: Set(1),
            role_id: Set(r_id),
            user_id: Set(user_id.clone()),
            ..Default::default()
        })
    }

    SysUserRole::insert_many(sys_role_user_list).exec(conn).await.unwrap();
    Ok(web::Json(ok_result_msg("更新用户角色信息成功!")))
}

#[get("/query_user_menu")]
pub async fn query_user_menu(req: HttpRequest, data: web::Data<AppState>) -> Either<Result<impl Responder>, Result<impl Responder>> {
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
        return Either::Left(Ok(web::Json(err_result_msg("the token format wrong"))));
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => { data }
        Err(err) => {
            return match err {
                WhoUnfollowedError::JwtTokenError(er) => {
                    Either::Left(Ok(web::Json(err_result_msg(er.as_str()))))
                }
                _ => {
                    Either::Left(Ok(web::Json(err_result_msg("other err"))))
                }
            };
        }
    };

    log::info!("query user menu params {:?}",jwt_token);

    let conn = &data.conn;

    if SysUser::find_by_id(jwt_token.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return Either::Left(Ok(web::Json(err_result_msg("用户不存在!"))));
    }

    let sys_menu_list: Vec<sys_menu::Model>;

    if SysUserRole::find().filter(sys_user_role::Column::UserId.eq(jwt_token.id.clone())).filter(sys_user_role::Column::RoleId.eq(1)).one(conn).await.unwrap_or_default().is_some() {
        sys_menu_list = SysMenu::find().all(conn).await.unwrap_or_default();
    } else {
        let sql_str = r#"select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = $1 order by u.id asc"#;
        sys_menu_list = SysMenu::find().from_raw_sql(Statement::from_sql_and_values(DatabaseBackend::MySql, sql_str, [jwt_token.id.clone().clone().into()])).all(conn).await.unwrap_or_default();
    }

    let mut btn_menu: HashSet<String> = HashSet::new();
    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

    for x in sys_menu_list {
        if x.menu_type != 3 {
            sys_menu_ids.insert(x.id);
            sys_menu_ids.insert(x.parent_id);
        }
        if x.api_url.len() > 0 {
            btn_menu.insert(x.api_url);
        }
    }

    let mut menu_ids = Vec::new();
    for id in sys_menu_ids {
        menu_ids.push(id)
    }
    let mut sys_menu: HashSet<MenuUserList> = HashSet::new();
    for y in SysMenu::find().filter(sys_menu::Column::Id.is_in(menu_ids)).filter(sys_menu::Column::StatusId.eq(1)).order_by_asc(sys_menu::Column::Sort).all(conn).await.unwrap_or_default() {
        sys_menu.insert(MenuUserList {
            id: y.id,
            parent_id: y.parent_id,
            name: y.menu_name,
            icon: y.menu_icon.unwrap_or_default(),
            api_url: y.api_url.clone(),
            menu_type: y.menu_type,
            path: y.menu_url,
        });
        if y.api_url.len() > 0 {
            btn_menu.insert(y.api_url.clone());
        }
    }

    let avatar = "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string();

    Either::Right(Ok(web::Json(ok_result_data(QueryUserMenuData { sys_menu, btn_menu, avatar, name: jwt_token.username }))))
}

// 查询用户列表
#[post("/user_list")]
pub async fn user_list(item: web::Json<UserListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query user_list params: {:?}", &item);
    let conn = &data.conn;

    let paginator = SysUser::find()
        .apply_if(item.mobile.clone(), |query, v| {
            query.filter(sys_user::Column::Mobile.eq(v))
        })
        .apply_if(item.status_id.clone(), |query, v| {
            query.filter(sys_user::Column::StatusId.eq(v))
        }).paginate(conn, item.page_size.clone());

    let total = paginator.num_items().await.unwrap_or_default();

    let mut list_data: Vec<UserListData> = Vec::new();

    for user in paginator.fetch_page(item.page_no.clone() - 1).await.unwrap_or_default() {
        list_data.push(UserListData {
            id: user.id,
            sort: user.sort,
            status_id: user.status_id,
            mobile: user.mobile,
            user_name: user.user_name,
            remark: user.remark.unwrap_or_default(),
            create_time: user.create_time.to_string(),
            update_time: user.update_time.to_string(),
        })
    }

    Ok(web::Json(ok_result_page(list_data, total)))
}

// 添加用户信息
#[post("/user_save")]
pub async fn user_save(item: web::Json<UserSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_save params: {:?}", &item);

    let user = item.0;
    let conn = &data.conn;

    let sys_user = sys_user::ActiveModel {
        id: NotSet,
        status_id: Set(user.status_id),
        sort: Set(user.sort),
        mobile: Set(user.mobile),
        user_name: Set(user.user_name),
        remark: Set(user.remark),
        ..Default::default()
    };

    SysUser::insert(sys_user).exec(conn).await.unwrap();
    Ok(web::Json(ok_result_msg("添加用户信息成功!")))
}

// 更新用户信息
#[post("/user_update")]
pub async fn user_update(item: web::Json<UserUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_update params: {:?}", &item);

    let user = item.0;
    let conn = &data.conn;

    if SysUser::find_by_id(user.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        // return  Ok(web::Json(err_result_msg("用户不存在!")));
        return Ok(web::Json(err_result_msg("用户不存在!")));
    }

    let sys_user = sys_user::ActiveModel {
        id: Set(user.id),
        status_id: Set(user.status_id),
        sort: Set(user.sort),
        mobile: Set(user.mobile),
        user_name: Set(user.user_name),
        remark: Set(user.remark),
        ..Default::default()
    };

    SysUser::update(sys_user).exec(conn).await.unwrap();
    Ok(web::Json(ok_result_msg("更新用户信息成功!")))
}

// 删除用户信息
#[post("/user_delete")]
pub async fn user_delete(item: web::Json<UserDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("user_delete params: {:?}", &item);
    let conn = &data.conn;

    let ids = item.ids.clone();
    for id in ids {
        if id != 1 {//id为1的用户为系统预留用户,不能删除
            let _ = SysUser::delete_by_id(id).exec(conn).await;
        }
    }

    Ok(web::Json(ok_result_msg("删除用户信息成功!")))
}

// 更新用户密码
#[post("/update_user_password")]
pub async fn update_user_password(item: web::Json<UpdateUserPwdReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update_user_pwd params: {:?}", &item);
    let conn = &data.conn;
    let user_pwd = item.0;

    let result = SysUser::find_by_id(user_pwd.id).one(conn).await.unwrap_or_default();
    if result.is_none() {
        return Ok(web::Json(err_result_msg("用户不存在!")));
    };

    let user = result.unwrap();
    if user.password == user_pwd.pwd {
        let mut s_user: sys_user::ActiveModel = user.into();
        s_user.password = Set(user_pwd.re_pwd);

        s_user.update(conn).await.unwrap();
        Ok(web::Json(ok_result_msg("更新用户密码成功!")))
    } else {
        Ok(web::Json(err_result_msg("旧密码不正确!")))
    }
}
