use actix_web::{post, Responder, Result, web};
use rbatis::rbdc::datetime::DateTime;
use crate::AppState;

use crate::model::menu::{SysMenu};
use crate::vo::{err_result_msg, err_result_page, handle_result, ok_result_page};
use crate::vo::menu_vo::{*};

// 查询菜单
#[post("/menu_list")]
pub async fn menu_list(item: web::Json<MenuListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_list params: {:?}", &item);
    let mut rb = &data.batis;

    // 菜单是树形结构不需要分页
    let result = SysMenu::select_all(&mut rb).await;

    let mut menu_list: Vec<MenuListData> = Vec::new();

    match result {
        Ok(sys_menu_list) => {
            for menu in sys_menu_list {
                menu_list.push(MenuListData {
                    id: menu.id.unwrap(),
                    sort: menu.sort,
                    status_id: menu.status_id,
                    parent_id: menu.parent_id,
                    menu_name: menu.menu_name.clone(),
                    label: menu.menu_name,
                    menu_url: menu.menu_url.unwrap_or_default(),
                    icon: menu.menu_icon.unwrap_or_default(),
                    api_url: menu.api_url.unwrap_or_default(),
                    remark: menu.remark.unwrap_or_default(),
                    menu_type: menu.menu_type,
                    create_time: menu.create_time.unwrap().0.to_string(),
                    update_time: menu.update_time.unwrap().0.to_string(),
                })
            }
            Ok(web::Json(ok_result_page(menu_list, 0)))
        }
        Err(err) => {
            Ok(web::Json(err_result_page(menu_list, err.to_string())))
        }
    }
}

// 添加菜单
#[post("/menu_save")]
pub async fn menu_save(item: web::Json<MenuSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_save params: {:?}", &item);
    let mut rb = &data.batis;

    let menu = item.0;

    let sys_menu = SysMenu {
        id: None,
        create_time: Some(DateTime::now()),
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id.unwrap_or(0),
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::insert(&mut rb, &sys_menu).await;

    Ok(web::Json(handle_result(result)))
}

// 更新菜单
#[post("/menu_update")]
pub async fn menu_update(item: web::Json<MenuUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = &data.batis;
    let menu = item.0;

    let sys_menu = SysMenu {
        id: Some(menu.id),
        create_time: None,
        update_time: Some(DateTime::now()),
        status_id: menu.status_id,
        sort: menu.sort,
        parent_id: menu.parent_id,
        menu_name: menu.menu_name,
        menu_url: menu.menu_url,
        api_url: menu.api_url,
        menu_icon: menu.icon,
        remark: menu.remark,
        menu_type: menu.menu_type,
    };

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    Ok(web::Json(handle_result(result)))
}

// 删除菜单信息
#[post("/menu_delete")]
pub async fn menu_delete(item: web::Json<MenuDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = &data.batis;

    //有下级的时候 不能直接删除
    let menus = SysMenu::select_by_column(&mut rb, "parent_id", &item.id).await.unwrap_or_default();

    if menus.len() > 0 {
        return Ok(web::Json(err_result_msg("有下级菜单,不能直接删除".to_string())));
    }

    let result = SysMenu::delete_by_column(&mut rb, "id", &item.id).await;

    Ok(web::Json(handle_result(result)))
}