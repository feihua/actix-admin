use actix_web::{post, web, Either, Responder, Result};
use diesel::associations::HasTable;
use diesel::sql_types::*;
use diesel::{sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, info};

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::SysMenu;
use crate::model::system::sys_role_menu_model::AddSysRoleMenu;
use crate::model::system::sys_role_model::*;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_role::dsl::sys_role;
use crate::schema::sys_role::*;
use crate::schema::sys_role_menu::dsl::sys_role_menu;
use crate::schema::sys_role_menu::{menu_id, role_id};
use crate::schema::sys_user_role::dsl::sys_user_role;
use crate::vo::system::sys_role_vo::*;

use crate::{schema, RB};

/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/add_sys_role")]
pub async fn add_sys_role(req: web::Json<AddRoleReq>) -> Result<impl Responder> {
    info!("add sys_role params: {:?}", &req);
    let item = req.0;

    let add_sys_role_param = AddSysRole {
        role_name: item.role_name,               //名称
        status_id: item.status_id,               //状态(1:正常，0:禁用)
        sort: item.sort,                         //排序
        remark: item.remark.unwrap_or_default(), //备注, //创建时间
        create_time: Default::default(),         //修改时间
        update_time: Default::default(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_role::table())
                .values(add_sys_role_param)
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
 *删除角色信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/delete_sys_role")]
pub async fn delete_sys_role(item: web::Json<DeleteRoleReq>) -> Result<impl Responder> {
    info!("delete sys_role params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let ids = item.ids.clone();
            //查询角色有没有被使用了,如果使用了就不能删除
            match sys_user_role
                .filter(schema::sys_user_role::role_id.eq_any(ids))
                .count()
                .get_result::<i64>(conn)
            {
                Ok(count) => {
                    if count != 0 {
                        error!("err:{}", "角色已被使用,不能删除".to_string());
                        return BaseResponse::<String>::err_result_msg(
                            "角色已被使用,不能删除".to_string(),
                        );
                    }
                    let result =
                        diesel::delete(sys_role.filter(id.eq_any(&item.ids))).execute(conn);
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
 *更新角色信息
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_sys_role")]
pub async fn update_sys_role(req: web::Json<UpdateRoleReq>) -> Result<impl Responder> {
    info!("update sys_role params: {:?}", &req);
    let item = req.0;

    let update_sys_role_param = UpdateSysRole {
        id: item.id,                             //主键
        role_name: item.role_name,               //名称
        status_id: item.status_id,               //状态(1:正常，0:禁用)
        sort: item.sort,                         //排序
        remark: item.remark.unwrap_or_default(), //备注
        create_time: Default::default(),         //创建时间
        update_time: Default::default(),         //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_role)
                .filter(id.eq(&item.id))
                .set(update_sys_role_param)
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
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_sys_role_status")]
pub async fn update_sys_role_status(
    item: web::Json<UpdateRoleStatusReq>,
) -> Result<impl Responder> {
    info!("update sys_role_status params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_role)
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
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_sys_role_detail")]
pub async fn query_sys_role_detail(item: web::Json<QueryRoleDetailReq>) -> Result<impl Responder> {
    info!("query sys_role_detail params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_role_sql = sql_query("SELECT * FROM sys_role WHERE id = ?");
            let result = sys_role_sql.bind::<Bigint, _>(&item.id).get_result::<SysRole>(conn);
            match result {
                Ok(x ) => {
                    let data = QueryRoleDetailResp {
                        id: x.id,                                 //主键
                        role_name: x.role_name,                            //名称
                        status_id: x.status_id,                            //状态(1:正常，0:禁用)
                        sort: x.sort,                                      //排序
                        remark: x.remark,                                  //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    };

                    BaseResponse::<QueryRoleDetailResp>::ok_result_data(data)
                }
                Err(err ) => {
                    BaseResponse::<QueryRoleDetailResp>::err_result_data(
                        QueryRoleDetailResp::new(),
                        err.to_string(),
                    )
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<QueryRoleDetailResp>::err_result_data(
                QueryRoleDetailResp::new(),
                err.to_string(),
            )
        }
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_sys_role_list")]
pub async fn query_sys_role_list(item: web::Json<QueryRoleListReq>) -> Result<impl Responder> {
    info!("query sys_role_list params: {:?}", &item);

    let mut query = sys_role::table().into_boxed();

    if let Some(i) = &item.role_name {
        query = query.filter(role_name.eq(i));
    }
    if let Some(i) = &item.status_id {
        query = query.filter(status_id.eq(i));
    }
    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            if let Ok(list) = query.load::<SysRole>(conn) {
                for x in list {
                    sys_role_list_data.push(RoleListDataResp {
                        id: x.id,                               //主键
                        role_name: x.role_name,                 //名称
                        status_id: x.status_id,                 //状态(1:正常，0:禁用)
                        sort: x.sort,                           //排序
                        remark: x.remark,                       //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
            }
            let total = 0;
            BaseResponse::<Vec<RoleListDataResp>>::ok_result_page(sys_role_list_data, total)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<Vec<RoleListDataResp>>::err_result_page(
                RoleListDataResp::new(),
                err.to_string(),
            )
        }
    }
}
/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/query_role_menu")]
pub async fn query_role_menu(
    item: web::Json<QueryRoleMenuReq>,
) -> Either<Result<impl Responder>, Result<impl Responder>> {
    info!("query_role_menu params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut menu_data_list: Vec<MenuList> = Vec::new();
            let mut role_menu_ids: Vec<i64> = Vec::new();
            // 查询所有菜单
            match sys_menu.load::<SysMenu>(conn) {
                Ok(menu_list) => {
                    for menu in menu_list {
                        menu_data_list.push(MenuList {
                            id: menu.id.clone(),
                            parent_id: menu.parent_id,
                            title: menu.menu_name.clone(),
                            key: menu.id.to_string(),
                            label: menu.menu_name,
                            is_penultimate: menu.parent_id == 2,
                        });
                        role_menu_ids.push(menu.id)
                    }
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    return Either::Left(BaseResponse::<String>::err_result_msg(err.to_string()));
                }
            }

            //不是超级管理员的时候,就要查询角色和菜单的关联
            if item.role_id != 1 {
                role_menu_ids.clear();

                match sys_role_menu
                    .filter(role_id.eq(item.role_id.clone()))
                    .select(menu_id)
                    .load::<i64>(conn)
                {
                    Ok(menu_ids) => role_menu_ids = menu_ids,
                    Err(err) => {
                        error!("err:{}", err.to_string());
                        return Either::Left(BaseResponse::<String>::err_result_msg(
                            err.to_string(),
                        ));
                    }
                }
            }

            Either::Right(BaseResponse::<QueryRoleMenuResp>::ok_result_data(
                QueryRoleMenuResp {
                    menu_ids: role_menu_ids,
                    menu_list: menu_data_list,
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
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/20 11:04:01
 */
#[post("/update_role_menu")]
pub async fn update_role_menu(item: web::Json<UpdateRoleMenuReq>) -> Result<impl Responder> {
    info!("update_role_menu params: {:?}", &item);

    let r_id = item.role_id.clone();
    let menu_ids = item.menu_ids.clone();

    match &mut RB.clone().get() {
        Ok(conn) => match diesel::delete(sys_role_menu.filter(role_id.eq(r_id))).execute(conn) {
            Ok(_) => {
                let mut role_menu: Vec<AddSysRoleMenu> = Vec::new();

                for m_id in menu_ids {
                    role_menu.push(AddSysRoleMenu {
                        menu_id: m_id.clone(),
                        role_id: r_id.clone(),
                        create_time: Default::default(),
                    })
                }

                let result = diesel::insert_into(sys_role_menu::table())
                    .values(&role_menu)
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
