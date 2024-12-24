use crate::AppState;
use actix_web::{post, web, Responder, Result};
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};

use crate::common::result::BaseResponse;
use crate::model::system::prelude::SysMenu;
use crate::model::system::sys_menu;
use crate::vo::system::sys_menu_vo::*;
/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/addMenu")]
pub async fn add_sys_menu(
    item: web::Json<AddMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("add sys_menu params: {:?}", &item);
    let conn = &data.conn;

    let req = item.0;

    let sys_menu = sys_menu::ActiveModel {
        id: NotSet,                    //主键
        menu_name: Set(req.menu_name), //菜单名称
        menu_type: Set(req.menu_type), //菜单类型(1：目录   2：菜单   3：按钮)
        status: Set(req.status),       //状态(1:正常，0:禁用)
        sort: Set(req.sort),           //排序
        parent_id: Set(req.parent_id), //父ID
        menu_url: Set(req.menu_url),   //路由路径
        api_url: Set(req.api_url),     //接口URL
        menu_icon: Set(req.menu_icon), //菜单图标
        remark: Set(req.remark),       //备注
        create_time: NotSet,           //创建时间
        update_time: NotSet,           //修改时间
    };

    let result = SysMenu::insert(sys_menu).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/deleteMenu")]
pub async fn delete_sys_menu(
    item: web::Json<DeleteMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("delete sys_menu params: {:?}", &item);
    let conn = &data.conn;
    let req = item.0;

    if SysMenu::find_by_id(req.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg("菜单不存在,不能删除!".to_string());
    }

    if SysMenu::find()
        .filter(sys_menu::Column::ParentId.eq(req.id.clone()))
        .count(conn)
        .await
        .unwrap_or_default()
        > 0
    {
        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除!".to_string());
    }

    let result = SysMenu::delete_many()
        .filter(sys_menu::Column::Id.eq(req.id))
        .exec(conn)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/updateMenu")]
pub async fn update_sys_menu(
    item: web::Json<UpdateMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu params: {:?}", &item);
    let conn = &data.conn;
    let req = item.0;

    if SysMenu::find_by_id(req.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::ok_result_msg("菜单信息不存在,不能更新!".to_string());
    }

    let sys_menu = sys_menu::ActiveModel {
        id: Set(req.id),               //主键
        menu_name: Set(req.menu_name), //菜单名称
        menu_type: Set(req.menu_type), //菜单类型(1：目录   2：菜单   3：按钮)
        status: Set(req.status),       //状态(1:正常，0:禁用)
        sort: Set(req.sort),           //排序
        parent_id: Set(req.parent_id), //父ID
        menu_url: Set(req.menu_url),   //路由路径
        api_url: Set(req.api_url),     //接口URL
        menu_icon: Set(req.menu_icon), //菜单图标
        remark: Set(req.remark),       //备注
        create_time: NotSet,           //创建时间
        update_time: NotSet,           //修改时间
    };

    let result = SysMenu::update(sys_menu).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/updateMenuStatus")]
pub async fn update_sys_menu_status(
    item: web::Json<UpdateMenuStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu_status params: {:?}", &item);
    let conn = &data.conn;
    let req = item.0;

    let result = SysMenu::update_many()
        .col_expr(sys_menu::Column::Status, Expr::value(req.status))
        .filter(sys_menu::Column::Id.eq(req.id))
        .exec(conn)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/queryMenuDetail")]
pub async fn query_sys_menu_detail(
    item: web::Json<QueryMenuDetailReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let conn = &data.conn;

    let result = SysMenu::find_by_id(item.id.clone()).one(conn).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_menu = QueryMenuDetailResp {
                id: x.id,                                   //主键
                menu_name: x.menu_name,                     //菜单名称
                menu_type: x.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
                status: x.status,                           //状态(1:正常，0:禁用)
                sort: x.sort,                               //排序
                parent_id: x.parent_id,                     //父ID
                menu_url: x.menu_url.unwrap_or_default(),   //路由路径
                api_url: x.api_url.unwrap_or_default(),     //接口URL
                menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                remark: x.remark.unwrap_or_default(),       //备注
                create_time: x.create_time.to_string(),     //创建时间
                update_time: x.update_time.to_string(),     //修改时间
            };

            BaseResponse::<QueryMenuDetailResp>::ok_result_data(sys_menu)
        }
        Err(err) => BaseResponse::<QueryMenuDetailResp>::err_result_data(
            QueryMenuDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/19 09:12:33
 */
#[post("/system/menu/queryMenuList")]
pub async fn query_sys_menu_list(
    item: web::Json<QueryMenuListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_menu_list params: {:?}", &item);
    let conn = &data.conn;

    let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();

    for x in SysMenu::find()
        .order_by_asc(sys_menu::Column::Sort)
        .all(conn)
        .await
        .unwrap_or_default()
    {
        sys_menu_list_data.push(MenuListDataResp {
            id: x.id,                                   //主键
            menu_name: x.menu_name,                     //菜单名称
            menu_type: x.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
            status: x.status,                           //状态(1:正常，0:禁用)
            sort: x.sort,                               //排序
            parent_id: x.parent_id,                     //父ID
            menu_url: x.menu_url.unwrap_or_default(),   //路由路径
            api_url: x.api_url.unwrap_or_default(),     //接口URL
            menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
            remark: x.remark.unwrap_or_default(),       //备注
            create_time: x.create_time.to_string(),     //创建时间
            update_time: x.update_time.to_string(),     //修改时间
        })
    }

    BaseResponse::<Vec<MenuListDataResp>>::ok_result_page(sys_menu_list_data, 0)
}
