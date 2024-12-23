use crate::AppState;
use actix_web::{post, web, Responder, Result};
use rbs::to_value;

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::Menu;
use crate::vo::system::sys_menu_vo::*;

/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/16 10:07:18
 */
#[post("/system/menu/addMenu")]
pub async fn add_sys_menu(
    item: web::Json<AddMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("add sys_menu params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let sys_menu = Menu {
        id: None,                 //主键
        menu_name: req.menu_name, //菜单名称
        menu_type: req.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
        status: req.status, //状态(1:正常，0:禁用)
        sort: req.sort,           //排序
        parent_id: req.parent_id, //父ID
        menu_url: req.menu_url,   //路由路径
        api_url: req.api_url,     //接口URL
        menu_icon: req.menu_icon, //菜单图标
        remark: req.remark,       //备注
        create_time: None,        //创建时间
        update_time: None,        //修改时间
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
 *date：2024/12/16 10:07:18
 */
#[post("/system/menu/deleteMenu")]
pub async fn delete_sys_menu(
    item: web::Json<DeleteMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("delete sys_menu params: {:?}", &item);
    let rb = &data.batis;

    //有下级的时候 不能直接删除
    let menus = Menu::select_by_column(rb, "parent_id", &item.id)
        .await
        .unwrap_or_default();

    if menus.len() > 0 {
        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除".to_string());
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
 *date：2024/12/16 10:07:18
 */
#[post("/system/menu/updateMenu")]
pub async fn update_sys_menu(
    item: web::Json<UpdateMenuReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let sys_menu = Menu {
        id: Some(req.id),         //主键
        menu_name: req.menu_name, //菜单名称
        menu_type: req.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
        status: req.status, //状态(1:正常，0:禁用)
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
 *date：2024/12/16 10:07:18
 */
#[post("/system/menu/updateMenuStatus")]
pub async fn update_sys_menu_status(
    item: web::Json<UpdateMenuStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("update sys_menu_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let param = vec![to_value!(req.status), to_value!(req.ids)];
    let result = rb
        .exec("update sys_menu set status = ? where id in ?", param)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/16 10:07:18
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
        Ok(d) => {
            let x = d.unwrap();

            let sys_menu = QueryMenuDetailResp {
                id: x.id.unwrap(),                                 //主键
                menu_name: x.menu_name,                            //菜单名称
                menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                status: x.status, //状态(1:正常，0:禁用)
                sort: x.sort,           //排序
                parent_id: x.parent_id, //父ID
                menu_url: x.menu_url.unwrap_or_default(), //路由路径
                api_url: x.api_url.unwrap_or_default(), //接口URL
                menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                remark: x.remark.unwrap_or_default(), //备注
                create_time: x.create_time.unwrap().0.to_string(), //创建时间
                update_time: x.update_time.unwrap().0.to_string(), //修改时间
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
 *date：2024/12/16 10:07:18
 */
#[post("/system/menu/queryMenuList")]
pub async fn query_sys_menu_list(
    item: web::Json<QueryMenuListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("query sys_menu_list params: {:?}", &item);
    let rb = &data.batis;

    let result = Menu::select_all(rb).await;

    let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();

    match result {
        Ok(d) => {
            for x in d {
                let sys_menu1 = MenuListDataResp {
                    id: x.id.unwrap(),                                 //主键
                    menu_name: x.menu_name,                            //菜单名称
                    menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                    status: x.status,
                    sort: x.sort,           //排序
                    parent_id: x.parent_id, //父ID
                    menu_url: x.menu_url.unwrap_or_default(), //路由路径
                    api_url: x.api_url.unwrap_or_default(), //接口URL
                    menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                    remark: x.remark.unwrap_or_default(), //备注
                    create_time: x.create_time.unwrap().0.to_string(), //创建时间
                    update_time: x.update_time.unwrap().0.to_string(), //修改时间
                };
                sys_menu_list_data.push(sys_menu1);
            }

            BaseResponse::<Vec<MenuListDataResp>>::ok_result_page(sys_menu_list_data, 0)
        }
        Err(err) => BaseResponse::<Vec<MenuListDataResp>>::err_result_page(
            MenuListDataResp::new(),
            err.to_string(),
        ),
    }
}
