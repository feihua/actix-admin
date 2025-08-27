use crate::AppState;
use actix_web::{post, web, Responder};
use rbs::value;
use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data};
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
) -> AppResult<impl Responder> {
    log::info!("add sys_menu params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let name = req.menu_name;
    if Menu::select_by_menu_name(rb, &name).await?.is_some() {
        return Err(AppError::BusinessError("菜单名称已存在"));
    }

    let menu_url = req.menu_url.clone();
    if menu_url.is_some() {
        if Menu::select_by_menu_url(rb, &menu_url.unwrap())
            .await?
            .is_some()
        {
            return Err(AppError::BusinessError("路由路径已存在"));
        }
    }

    let sys_menu = Menu {
        id: None,                                      //主键
        menu_name: name,                               //菜单名称
        menu_type: req.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
        visible: req.visible,                         //菜单状态（0:隐藏, 显示:1）
        status: req.status,                           //状态(1:正常，0:禁用)
        sort: req.sort,                               //排序
        parent_id: req.parent_id.unwrap_or_default(), //上级菜单
        menu_url: req.menu_url,                       //路由路径
        api_url: req.api_url,                         //接口URL
        menu_icon: req.menu_icon,                     //菜单图标
        remark: req.remark,                           //备注
        create_time: None,                             //创建时间
        update_time: None,                             //修改时间
    };

    Menu::insert(rb, &sys_menu).await?;

    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("delete sys_menu params: {:?}", &item);
    let rb = &data.batis;

    //有下级的时候 不能直接删除
    if select_count_menu_by_parent_id(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("存在子菜单,不允许删除"));
    }

    if select_count_menu_by_menu_id(rb, &item.id).await? > 0 {
        return Err(AppError::BusinessError("菜单已分配,不允许删除"));
    }

    Menu::delete_by_map(rb, value! {"id": &item.id}).await?;

    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("update sys_menu params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if Menu::select_by_id(rb, &req.id).await?.is_none() {
        return Err(AppError::BusinessError("菜单信息不存在"));
    }

    if let Some(x) = Menu::select_by_menu_name(rb, &req.menu_name).await? {
        if x.id.unwrap_or_default() != req.id {
            return Err(AppError::BusinessError("菜单名称已存在"))
        }
    }

    let menu_url = req.menu_url.clone();
    if menu_url.is_some() {
        if let Some(x) = Menu::select_by_menu_url(rb, &menu_url.unwrap()).await? {
            if x.id.unwrap_or_default() != req.id {
                return Err(AppError::BusinessError("路由路径已存在"));
            }
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
        create_time: None,         //创建时间
        update_time: None,         //修改时间
    };

    Menu::update_by_map(rb, &sys_menu, value! {"id": &sys_menu.id}).await?;

    ok_result()
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
) -> AppResult<impl Responder> {
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

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;

    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let rb = &data.batis;

    match Menu::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("菜单信息不存在")),
        Some(x) => {
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

            ok_result_data(sys_menu)
        }
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
) -> AppResult<impl Responder> {
    log::info!("query sys_menu_list params: {:?}", &item);
    let rb = &data.batis;

    let list = Menu::select_all(rb).await?;

    let mut menu_list: Vec<MenuListDataResp> = Vec::new();
    for x in list {
        menu_list.push(MenuListDataResp {
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
        })
    }

    ok_result_data(menu_list)
}

/*
 *查询菜单信息(排除按钮)
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/menu/queryMenuList")]
pub async fn query_sys_menu_list_simple(data: web::Data<AppState>) -> AppResult<impl Responder> {
    let rb = &data.batis;

    let list = Menu::select_menu_list(rb).await?;

    let mut menu_list: Vec<MenuListSimpleDataResp> = Vec::new();
    for x in list {
        menu_list.push(MenuListSimpleDataResp {
            id: x.id.unwrap_or_default(), //主键
            menu_name: x.menu_name,       //菜单名称
            parent_id: x.parent_id,       //父ID
        })
    }

    ok_result_data(menu_list)
}
