use actix_web::http::header;
use actix_web::{get, post, web, Either, HttpRequest, Responder, Result};
use diesel::associations::HasTable;
use diesel::sql_types::*;
use diesel::{sql_query, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use log::{debug, error, info, warn};
use std::collections::HashSet;

use crate::common::error::WhoUnfollowedError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::{StringColumn, SysMenu};
use crate::model::system::sys_role_model::SysRole;
use crate::model::system::sys_user_model::*;
use crate::model::system::sys_user_role_model::{AddSysUserRole, SysUserRole};
use crate::schema::sys_menu::api_url;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_role::dsl::sys_role;
use crate::schema::sys_user::dsl::sys_user;
use crate::schema::sys_user::*;
use crate::schema::sys_user_role::dsl::sys_user_role;
use crate::schema::sys_user_role::{role_id, user_id};
use crate::utils::jwt_util::JWTToken;
use crate::vo::system::sys_user_vo::*;

use crate::{schema, RB};

/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/add_sys_user")]
pub async fn add_sys_user(req: web::Json<AddUserReq>) -> Result<impl Responder> {
    info!("add sys_user params: {:?}", &req);
    let item = req.0;

    let add_sys_user_param = AddSysUser {
        mobile: item.mobile,             //手机
        user_name: item.user_name,       //姓名
        password: item.password,         //密码
        status_id: item.status_id,       //状态(1:正常，0:禁用)
        sort: item.sort,                 //排序
        remark: item.remark,             //备注, //创建时间
        create_time: Default::default(), //修改时间
        update_time: Default::default(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_user::table())
                .values(add_sys_user_param)
                .execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *删除用户信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/delete_sys_user")]
pub async fn delete_sys_user(item: web::Json<DeleteUserReq>) -> Result<impl Responder> {
    info!("delete sys_user params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let ids = item.ids.clone();
            //id为1的用户为系统预留用户,不能删除
            let mut delete_ids = vec![];
            for delete_id in ids {
                if delete_id == 1 {
                    warn!("err:{}", "不能删除超级管理员".to_string());
                    continue;
                }
                delete_ids.push(delete_id)
            }

            if delete_ids.len() == 0 {
                return BaseResponse::<String>::ok_result();
            }

            let query = diesel::delete(sys_user.filter(id.eq_any(delete_ids)));
            debug!(
                "SQL: {}",
                diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
            );
            let result = query.execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_sys_user")]
pub async fn update_sys_user(req: web::Json<UpdateUserReq>) -> Result<impl Responder> {
    info!("update sys_user params: {:?}", &req);
    let item = req.0;

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");

            match user_sql
                .bind::<Bigint, _>(item.id)
                .get_result::<SysUser>(conn)
            {
                Ok(s_user) => {
                    let s_user = UpdateSysUser {
                        id: item.id.clone(),
                        status_id: item.status_id,
                        sort: item.sort,
                        mobile: item.mobile,
                        user_name: item.user_name,
                        remark: item.remark,
                        create_time: s_user.create_time,
                        update_time: Default::default(),
                    };

                    let query = diesel::update(sys_user.filter(id.eq(item.id.clone()))).set(s_user);
                    debug!(
                        "SQL:{}",
                        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
                    );
                    let result = query.execute(conn);
                    match result {
                        Ok(_u) => BaseResponse::<String>::ok_result(),
                        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                    }
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    BaseResponse::<String>::err_result_msg(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_sys_user_status")]
pub async fn update_sys_user_status(
    item: web::Json<UpdateUserStatusReq>,
) -> Result<impl Responder> {
    info!("update sys_user_status params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_user)
                .filter(id.eq_any(&item.ids))
                .set(status_id.eq(item.status))
                .execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *更新用户密码
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_user_password")]
pub async fn update_user_password(item: web::Json<UpdateUserPwdReq>) -> Result<impl Responder> {
    info!("update_user_pwd params: {:?}", &item);

    let user_pwd = item.0;

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");
            match user_sql
                .bind::<Bigint, _>(user_pwd.id)
                .get_result::<SysUser>(conn)
            {
                Ok(user) => {
                    if user.password != user_pwd.pwd {
                        error!("err:{}", "旧密码不正确".to_string());
                        return BaseResponse::<String>::err_result_msg("旧密码不正确".to_string());
                    }
                    let result = diesel::update(sys_user.filter(id.eq(user_pwd.id.clone())))
                        .set(password.eq(&user_pwd.re_pwd))
                        .execute(conn);
                    match result {
                        Ok(_u) => BaseResponse::<String>::ok_result(),
                        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                    }
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    BaseResponse::<String>::err_result_msg(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_sys_user_detail")]
pub async fn query_sys_user_detail(item: web::Json<QueryUserDetailReq>) -> Result<impl Responder> {
    info!("query sys_user_detail params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_user_sql = sql_query("SELECT * FROM sys_user WHERE id = ?");
            let result = sys_user_sql.bind::<Bigint, _>(&item.id).get_result::<SysUser>(conn);
            match result {
                Ok(x ) => {
                    let data = QueryUserDetailResp {
                        id: x.id,                                 //主键
                        mobile: x.mobile,                                  //手机
                        user_name: x.user_name,                            //姓名
                        status_id: x.status_id,                            //状态(1:正常，0:禁用)
                        sort: x.sort,                                      //排序
                        remark: x.remark.unwrap_or_default(),              //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    };

                    BaseResponse::<QueryUserDetailResp>::ok_result_data(data)
                }
                Err(err ) => {
                    error!("err:{}", err.to_string());
                    BaseResponse::<QueryUserDetailResp>::err_result_data(
                        QueryUserDetailResp::new(),
                        err.to_string(),
                    )
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<QueryUserDetailResp>::err_result_data(
                QueryUserDetailResp::new(),
                err.to_string(),
            )
        }
    }
}

/*
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_sys_user_list")]
pub async fn query_sys_user_list(item: web::Json<QueryUserListReq>) -> Result<impl Responder> {
    info!("query sys_user_list params: {:?}", &item);

    let mut query = sys_user::table().into_boxed();

    if let Some(i) = &item.status_id {
        query = query.filter(status_id.eq(i));
    }
    if let Some(i) = &item.mobile {
        query = query.filter(mobile.eq(i));
    }
    if let Some(i) = &item.user_name {
        query = query.filter(user_name.eq(i));
    }

    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            if let Ok(list) = query.load::<SysUser>(conn) {
                for x in list {
                    sys_user_list_data.push(UserListDataResp {
                        id: x.id,                               //主键
                        mobile: x.mobile,                       //手机
                        user_name: x.user_name,                 //姓名
                        status_id: x.status_id,                 //状态(1:正常，0:禁用)
                        sort: x.sort,                           //排序
                        remark: x.remark.unwrap_or_default(),   //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
            }
            let total = 0;
            BaseResponse::<Vec<UserListDataResp>>::ok_result_page(sys_user_list_data, total)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<Vec<UserListDataResp>>::err_result_page(
                UserListDataResp::new(),
                err.to_string(),
            )
        }
    }
}

/*
 *后台用户登录
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/login")]
pub async fn login(item: web::Json<UserLoginReq>) -> Result<impl Responder> {
    info!("user login params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let query = sys_user.filter(mobile.eq(&item.mobile));
            debug!(
                "SQL: {}",
                diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
            );

            if let Ok(user) = query.first::<SysUser>(conn) {
                info!("select_by_mobile: {:?}", user);

                if user.password.ne(&item.password) {
                    return BaseResponse::<String>::err_result_msg("密码不正确".to_string());
                }

                let btn_menu = query_btn_menu(user.id);

                if btn_menu.len() == 0 {
                    return BaseResponse::<String>::err_result_msg(
                        "用户没有分配角色或者菜单,不能登录".to_string(),
                    );
                }

                match JWTToken::new(user.id, &user.user_name, btn_menu).create_token("123") {
                    Ok(token) => BaseResponse::<String>::ok_result_data(token),
                    Err(err) => {
                        let er = match err {
                            WhoUnfollowedError::JwtTokenError(s) => s,
                            _ => "no math error".to_string(),
                        };

                        error!("err:{}", er.to_string());
                        BaseResponse::<String>::err_result_msg(er)
                    }
                }
            } else {
                error!("err:{}", "根据手机号查询用户异常".to_string());
                BaseResponse::<String>::err_result_msg("根据手机号查询用户异常".to_string())
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *后台用户登录
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
fn query_btn_menu(u_id: i64) -> Vec<String> {
    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_role_sql =
                sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
            match user_role_sql
                .bind::<Bigint, _>(&u_id)
                .get_result::<SysUserRole>(conn)
            {
                Ok(_) => {
                    let sys_menu_result = sys_menu.select(api_url).load::<String>(conn);
                    match sys_menu_result {
                        Ok(btn) => btn,
                        Err(_) => Vec::new(),
                    }
                }
                Err(_) => {
                    let result = sql_query(
                        "select u.api_url from sys_user_role t \
                    left join sys_role usr on t.role_id = usr.id \
                    left join sys_role_menu srm on usr.id = srm.role_id \
                    left join sys_menu u on srm.menu_id = u.id \
                    where t.user_id = ?",
                    )
                    .bind::<Bigint, _>(&u_id)
                    .load::<StringColumn>(conn);

                    match result {
                        Ok(btn_list) => {
                            let mut btn_list_data: Vec<String> = Vec::new();
                            for x in btn_list {
                                if x.api_url.len() != 0 {
                                    btn_list_data.push(x.api_url);
                                }
                            }
                            return btn_list_data;
                        }
                        Err(_) => Vec::new(),
                    }
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Vec::new()
        }
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_user_role")]
pub async fn query_user_role(
    item: web::Json<QueryUserRoleReq>,
) -> Either<Result<impl Responder>, Result<impl Responder>> {
    info!("query_user_role params: {:?}", item);

    let mut user_role_ids: Vec<i64> = Vec::new();
    let mut sys_role_list: Vec<RoleList> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            if let Ok(ids) = sys_user_role
                .filter(user_id.eq(&item.user_id))
                .select(role_id)
                .load::<i64>(conn)
            {
                user_role_ids = ids
            }

            let sys_role_result = sys_role.load::<SysRole>(conn);

            if let Ok(role_list) = sys_role_result {
                for x in role_list {
                    sys_role_list.push(RoleList {
                        id: x.id,
                        status_id: x.status_id,
                        sort: x.sort,
                        role_name: x.role_name,
                        remark: x.remark,
                        create_time: x.create_time.to_string(),
                        update_time: x.update_time.to_string(),
                    });
                }
            }
            Either::Right(BaseResponse::<QueryUserRoleResp>::ok_result_data(
                QueryUserRoleResp {
                    role_list: sys_role_list,
                    role_ids: user_role_ids,
                },
            ))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

/*
 *更新用户角色
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_user_role")]
pub async fn update_user_role(item: web::Json<UpdateUserRoleReq>) -> Result<impl Responder> {
    info!("update_user_role params: {:?}", item);
    let user_role = item.0;
    let u_id = user_role.user_id;
    let role_ids = user_role.role_ids;

    if u_id == 1 {
        return BaseResponse::<String>::err_result_msg("不能修改超级管理员的角色".to_string());
    }

    match &mut RB.clone().get() {
        Ok(conn) => match diesel::delete(sys_user_role.filter(user_id.eq(u_id))).execute(conn) {
            Ok(_) => {
                let mut sys_role_user_list: Vec<AddSysUserRole> = Vec::new();
                for r_id in role_ids {
                    sys_role_user_list.push(AddSysUserRole {
                        role_id: r_id,
                        user_id: u_id.clone(),
                        create_time: Default::default(),
                    })
                }
                let result = diesel::insert_into(sys_user_role::table())
                    .values(&sys_role_user_list)
                    .execute(conn);
                match result {
                    Ok(_u) => BaseResponse::<String>::ok_result(),
                    Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                }
            }
            Err(err) => {
                error!("err:{}", err.to_string());
                BaseResponse::<String>::err_result_msg(err.to_string())
            }
        },
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *查询用户菜单
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[get("/query_user_menu")]
pub async fn query_user_menu(
    req: HttpRequest,
) -> Either<Result<impl Responder>, Result<impl Responder>> {
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
        return Either::Left(BaseResponse::<String>::err_result_msg(
            "the token format wrong".to_string(),
        ));
    }

    let jwt_token = match JWTToken::verify("123", split_vec[1]) {
        Ok(data) => data,
        Err(err) => {
            return match err {
                WhoUnfollowedError::JwtTokenError(er) => {
                    Either::Left(BaseResponse::<String>::err_result_msg(er.to_string()))
                }
                _ => Either::Left(BaseResponse::<String>::err_result_msg(
                    "other err".to_string(),
                )),
            };
        }
    };

    info!("query user menu by jwt_token {:?}", jwt_token);

    match &mut RB.clone().get() {
        Ok(conn) => {
            return match sql_query("select * from sys_user where id = ?")
                .bind::<Bigint, _>(jwt_token.id)
                .get_result::<SysUser>(conn)
            {
                Ok(user) => {
                    let user_role_sql =
                        sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
                    let sys_menu_list: Vec<SysMenu>;
                    match user_role_sql.bind::<Bigint, _>(&user.id).get_result::<SysUserRole>(conn) {
                        Ok(_) => {
                            match sys_menu.load::<SysMenu>(conn) {
                                Ok(s_menus) => {
                                    sys_menu_list = s_menus;
                                }
                                Err(err) => {
                                    error!("err:{}", err.to_string());
                                    return Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()));
                                }
                            }
                        }
                        Err(_) => {
                            match sql_query("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc")
                                .bind::<Bigint, _>(&jwt_token.id)
                                .load::<SysMenu>(conn) {
                                Ok(s_menus) => {
                                    sys_menu_list = s_menus;
                                }
                                Err(err) => {
                                    error!("err:{}", err.to_string());
                                    return Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()));
                                }
                            }
                        }
                    }

                    let mut sys_user_menu_list: Vec<MenuList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.parent_id.clone());
                            sys_menu_ids.insert(x.id.clone());
                        }

                        if x.api_url.len() != 0 {
                            btn_menu.push(x.api_url);
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for x in sys_menu_ids {
                        menu_ids.push(x)
                    }
                    let r: QueryResult<Vec<SysMenu>> = sys_menu
                        .filter(schema::sys_menu::id.eq_any(menu_ids))
                        .filter(schema::sys_menu::status_id.eq(1))
                        .order(crate::schema::sys_menu::sort.asc())
                        .distinct()
                        .load::<SysMenu>(conn);
                    match r {
                        Ok(menu_list1) => {
                            for x in menu_list1 {
                                sys_user_menu_list.push(MenuList {
                                    id: x.id,
                                    parent_id: x.parent_id,
                                    name: x.menu_name,
                                    icon: x.menu_icon,
                                    api_url: x.api_url.clone(),
                                    menu_type: x.menu_type,
                                    path: x.menu_url,
                                });
                            }
                        }
                        Err(err) => {
                            error!("err:{}", err.to_string());
                            return Either::Left(BaseResponse::<String>::err_result_msg(
                                err.to_string(),
                            ));
                        }
                    }

                    Either::Right(BaseResponse::<QueryUserMenuResp>::ok_result_data(QueryUserMenuResp {
                        sys_menu: sys_user_menu_list,
                        btn_menu,
                        avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                        name: user.user_name,
                    }))
                }

                Err(err) => {
                    error!("err:{}", err.to_string());
                    Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()))
                }
            };
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}
