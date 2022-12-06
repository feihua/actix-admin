use actix_web::{post, Responder, Result, web};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::{PageRequest};
use crate::AppState;
use crate::model::entity::{SysRole, query_menu_by_role, SysMenu, SysMenuRole};
use crate::vo::handle_result;
use crate::vo::role_vo::*;


#[post("/role_list")]
pub async fn role_list(item: web::Json<RoleListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("role_list params: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysRole::select_page(&mut rb, &PageRequest::new(item.page_no, item.page_size)).await;

    let resp = match result {
        Ok(d) => {
            let total = d.total;
            let page_no = d.page_no;
            let page_size = d.page_size;

            let mut role_list: Vec<RoleListData> = Vec::new();

            for x in d.records {
                role_list.push(RoleListData {
                    id: x.id.unwrap(),
                    sort: x.sort.unwrap(),
                    status_id: x.status_id.unwrap(),
                    role_name: x.role_name.unwrap_or_default(),
                    remark: x.remark.unwrap_or_default(),
                    create_time: x.gmt_create.unwrap().0.to_string(),
                    update_time: x.gmt_modified.unwrap().0.to_string(),
                })
            }

            RoleListResp {
                msg: "successful".to_string(),
                code: 0,
                page_no,
                page_size,
                success: true,
                total,
                data: Some(role_list),
            }
        }
        Err(err) => {
            RoleListResp {
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


#[post("/role_save")]
pub async fn role_save(item: web::Json<RoleSaveReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    println!("model: {:?}", &item);
    let mut rb = &data.batis;

    let role = item.0;

    let sys_role = SysRole {
        id: None,
        gmt_create: Some(FastDateTime::now()),
        gmt_modified: None,
        status_id: Some(1),
        sort: Some(role.sort),
        role_name: Some(role.role_name),
        remark: Some(role.remark),
    };

    let result = SysRole::insert(&mut rb, &sys_role).await;

    Ok(web::Json(handle_result(result)))
}


#[post("/role_update")]
pub async fn role_update(item: web::Json<RoleUpdateReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    println!("item: {:?}", &item);
    let mut rb = &data.batis;
    let role = item.0;

    let sys_role = SysRole {
        id: Some(role.id),
        gmt_create: None,
        gmt_modified: Some(FastDateTime::now()),
        status_id: Some(role.status_id),
        sort: Some(role.sort),
        role_name: Some(role.role_name),
        remark: Some(role.remark),
    };

    let result = SysRole::update_by_column(&mut rb, &sys_role, "id").await;

    Ok(web::Json(handle_result(result)))
}


#[post("/role_delete")]
pub async fn role_delete(item: web::Json<RoleDeleteReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    println!("item: {:?}", &item);
    let mut rb = &data.batis;

    let result = SysRole::delete_in_column(&mut rb, "id", &item.ids).await;

    Ok(web::Json(handle_result(result)))
}


#[post("/query_role_menu")]
pub async fn query_role_menu(item: web::Json<QueryRoleMenuReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query_role_menu params: {:?}", &item);
    let mut rb = &data.batis;

    let role_menu_list = query_menu_by_role(&mut rb, item.role_id).await;

    let menu_list = SysMenu::select_all(&mut rb).await;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();

    for y in menu_list.unwrap_or_default() {
        let x = y.clone();
        menu_data_list.push(MenuDataList {
            id: x.id.unwrap(),
            parent_id: x.parent_id.unwrap(),
            title: x.menu_name.unwrap_or_default(),
            key: "x.id.to_string()".to_string(),
        });
    }

    let resp = QueryRoleMenuResp {
        msg: "successful".to_string(),
        code: 0,
        data: QueryRoleMenuData {
            role_menus: role_menu_list.unwrap_or_default(),
            menu_list: menu_data_list,
        },
    };

    Ok(web::Json(resp))
}


#[post("/update_role_menu")]
pub async fn update_role_menu(item: web::Json<UpdateRoleMenuReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update_role_menu params: {:?}", &item);
    let role_id = item.role_id;

    let mut rb = &data.batis;

    SysMenuRole::delete_by_column(&mut rb, "role_id", &role_id).await.expect("删除角色菜单异常");

    let mut menu_role: Vec<SysMenuRole> = Vec::new();

    for x in &item.menu_ids {
        menu_role.push(SysMenuRole {
            id: None,
            gmt_create: Some(FastDateTime::now()),
            gmt_modified: Some(FastDateTime::now()),
            status_id: Some(1),
            sort: Some(1),
            menu_id: Some(*x),
            role_id: Some(role_id),
        })
    }

    let result = SysMenuRole::insert_batch(&mut rb, &menu_role, item.menu_ids.len() as u64).await;

    Ok(web::Json(handle_result(result)))
}
