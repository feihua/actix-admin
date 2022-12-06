use actix_web::{post, Responder, Result, web};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use crate::AppState;

use crate::model::entity::{SysMenu};
use crate::vo::handle_result;
use crate::vo::menu_vo::{*};


#[post("/menu_list")]
pub async fn menu_list(item: web::Json<MenuListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_list params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysMenu::select_page(&mut rb, &PageRequest::new(1, 1000)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;

            let mut menu_list: Vec<MenuListData> = Vec::new();

            for x in d.records {
                menu_list.push(MenuListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    parent_id: x.parent_id.unwrap(),
                    menu_name: x.menu_name.as_ref().unwrap().to_string(),
                    label: x.menu_name.unwrap_or_default(),
                    menu_url: x.menu_url.unwrap_or_default(),
                    icon: x.menu_icon.unwrap_or_default(),
                    api_url: x.api_url.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    menu_type: x.menu_type.unwrap(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }
            MenuListResp {
                msg: "successful".to_string(),
                code: 0,
                total,
                data: Some(menu_list),
            }
        }
        Err(err) => {
            MenuListResp {
                msg: err.to_string(),
                code: 1,
                total: 0,
                data: None,
            }
        }
    };


    Ok(web::Json(resp))
}

#[post("/menu_save")]
pub async fn menu_save(item: web::Json<MenuSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_save params: {:?}", &item);
    let mut rb = &data.batis;

    let menu = item.0;

    let role = SysMenu {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: None,
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
    };

    let result = SysMenu::insert(&mut rb, &role).await;

    Ok(web::Json(handle_result(result)))
}

#[post("/menu_update")]
pub async fn menu_update(item: web::Json<MenuUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_update params: {:?}", &item);
    let mut rb = &data.batis;
    let menu = item.0;

    let sys_menu = SysMenu {
        id: Some(menu.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(menu.status_id),
        sort: Some(menu.sort),
        parent_id: Some(menu.parent_id),
        menu_name: Some(menu.menu_name),
        menu_url: Some(menu.menu_url),
        api_url: Some(menu.api_url),
        menu_icon: Some(menu.icon),
        remark: Some(menu.remark),
        menu_type: Some(menu.menu_type),
    };

    let result = SysMenu::update_by_column(&mut rb, &sys_menu, "id").await;

    Ok(web::Json(handle_result(result)))
}


#[post("/menu_delete")]
pub async fn menu_delete(item: web::Json<MenuDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("menu_delete params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysMenu::delete_in_column(&mut rb, "id", &item.ids).await;

    Ok(web::Json(handle_result(result)))
}