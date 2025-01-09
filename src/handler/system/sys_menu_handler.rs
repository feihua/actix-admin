use crate::AppState;
use actix_web::{post, web, Responder, Result};
use rbs::to_value;

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::{select_count_menu_by_parent_id, Menu};
use crate::model::system::sys_role_menu_model::select_count_menu_by_menu_id;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_menu_vo::*;

/*
 *添加菜单信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/addMenu")]
pub async fn add_sys_menu(
    item: web::Json<AddMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("add sys_menu params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let res = Menu::select_by_menu_name(rb, &req.menu_name).await;
    match res {
        Ok(opt_menu) => {
            if opt_menu.is_some() {
                return BaseResponse::<String>::err_result_msg("菜单名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let menu_url = req.menu_url.clone();
    if menu_url.is_some() {
        let res = Menu::select_by_menu_url(rb, &menu_url.unwrap()).await;
        match res {
            Ok(opt_menu) => {
                if opt_menu.is_some() {
                    return BaseResponse::<String>::err_result_msg("路由路径已存在".to_string());
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        }
    }

    let sys_menu = Menu {
        id: None,                                     //主键
        menu_name: req.menu_name,                     //菜单名称
        menu_type: req.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
        visible: req.visible,                         //菜单状态（0:隐藏, 显示:1）
        status: req.status,                           //状态(1:正常，0:禁用)
        sort: req.sort,                               //排序
        parent_id: req.parent_id.unwrap_or_default(), //上级菜单
        menu_url: req.menu_url,                       //路由路径
        api_url: req.api_url,                         //接口URL
        menu_icon: req.menu_icon,                     //菜单图标
        remark: req.remark,                           //备注
        create_time: None,                            //创建时间
        update_time: None,                            //修改时间
    };

    let result = Menu::insert(rb, &sys_menu).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/deleteMenu")]
pub async fn delete_sys_menu(
    item: web::Json<DeleteMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("delete sys_menu params: {:?}", &item);
    let rb = &data.batis;

    //有下级的时候 不能直接删除
    let count = select_count_menu_by_parent_id(rb, &item.id)
        .await
        .unwrap_or_default();

    if count > 0 {
        return BaseResponse::<String>::err_result_msg("存在子菜单,不允许删除".to_string());
    }
    let count1 = select_count_menu_by_menu_id(rb, &item.id)
        .await
        .unwrap_or_default();

    if count1 > 0 {
        return BaseResponse::<String>::err_result_msg("菜单已分配,不允许删除".to_string());
    }

    let result = Menu::delete_by_column(rb, "id", &item.id).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/updateMenu")]
pub async fn update_sys_menu(
    item: web::Json<UpdateMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let res = Menu::select_by_menu_name(rb, &req.menu_name).await;
    match res {
        Ok(opt_menu) => {
            if opt_menu.is_some() && opt_menu.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg("菜单名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let menu_url = req.menu_url.clone();
    if menu_url.is_some() {
        let res = Menu::select_by_menu_url(rb, &menu_url.unwrap()).await;
        match res {
            Ok(opt_menu) => {
                if opt_menu.is_some() && opt_menu.unwrap().id.unwrap_or_default() != req.id {
                    return BaseResponse::<String>::err_result_msg("路由路径已存在".to_string());
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        }
    }

    let sys_menu = Menu {
        id: Some(req.id),         //主键
        menu_name: req.menu_name, //菜单名称
        menu_type: req.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
        visible: req.visible,     //菜单状态（0:隐藏, 显示:1）
        status: req.status,       //状态(1:正常，0:禁用)
        sort: req.sort,           //排序
        parent_id: req.parent_id, //父ID
        menu_url: req.menu_url,   //路由路径
        api_url: req.api_url,     //接口URL
        menu_icon: req.menu_icon, //菜单图标
        remark: req.remark,       //备注
        create_time: None,        //创建时间
        update_time: None,        //修改时间
    };

    let result = Menu::update_by_column(rb, &sys_menu, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/updateMenuStatus")]
pub async fn update_sys_menu_status(
    item: web::Json<UpdateMenuStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_menu set status = ? where id in ({})",
        req.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![to_value!(req.status)];
    param.extend(req.ids.iter().map(|&id| to_value!(id)));
    let result = rb.exec(&update_sql, param).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/queryMenuDetail")]
pub async fn query_sys_menu_detail(
    item: web::Json<QueryMenuDetailReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let rb = &data.batis;

    let result = Menu::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_menu) => {
            if opt_sys_menu.is_none() {
                return BaseResponse::<QueryMenuDetailResp>::err_result_data(
                    QueryMenuDetailResp::new(),
                    "菜单信息不存在".to_string(),
                );
            }
            let x = opt_sys_menu.unwrap();

            let sys_menu = QueryMenuDetailResp {
                id: x.id.unwrap_or_default(),               //主键
                menu_name: x.menu_name,                     //菜单名称
                menu_type: x.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
                visible: x.visible,                         //菜单状态（0:隐藏, 显示:1）
                status: x.status,                           //状态(1:正常，0:禁用)
                sort: x.sort,                               //排序
                parent_id: x.parent_id,                     //父ID
                menu_url: x.menu_url.unwrap_or_default(),   //路由路径
                api_url: x.api_url.unwrap_or_default(),     //接口URL
                menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                remark: x.remark.unwrap_or_default(),       //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
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
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/queryMenuList")]
pub async fn query_sys_menu_list(
    item: web::Json<QueryMenuListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_menu_list params: {:?}", &item);
    let rb = &data.batis;

    let result = Menu::select_all(rb).await;

    match result {
        Ok(list) => {
            let mut menu_list: Vec<MenuListDataResp> = Vec::new();
            for x in list {
                menu_list.push(MenuListDataResp {
                    id: x.id.unwrap_or_default(),               //主键
                    menu_name: x.menu_name,                     //菜单名称
                    menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                    visible: x.visible,     //菜单状态（0:隐藏, 显示:1）
                    status: x.status,       //状态(1:正常，0:禁用)
                    sort: x.sort,           //排序
                    parent_id: x.parent_id, //父ID
                    menu_url: x.menu_url.unwrap_or_default(), //路由路径
                    api_url: x.api_url.unwrap_or_default(), //接口URL
                    menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                    remark: x.remark.unwrap_or_default(), //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_data(menu_list)
        }
        Err(err) => BaseResponse::err_result_data(MenuListDataResp::new(), err.to_string()),
    }
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/queryMenuList")]
pub async fn query_sys_menu_list_simple(data: web::Data<AppState>) -> Result<impl Responder> {
    let rb = &data.batis;

    let result = Menu::select_menu_list(rb).await;

    match result {
        Ok(list) => {
            let mut menu_list: Vec<MenuListSimpleDataResp> = Vec::new();
            for x in list {
                menu_list.push(MenuListSimpleDataResp {
                    id: x.id.unwrap_or_default(), //主键
                    menu_name: x.menu_name,       //菜单名称
                    parent_id: x.parent_id,       //父ID
                })
            }

            BaseResponse::ok_result_data(menu_list)
        }
        Err(err) => BaseResponse::err_result_data(MenuListSimpleDataResp::new(), err.to_string()),
    }
}
