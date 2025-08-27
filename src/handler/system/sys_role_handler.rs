use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbs::value;
use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_menu_model::Menu;
use crate::model::system::sys_role_dept_model::RoleDept;
use crate::model::system::sys_role_menu_model::{query_menu_by_role, RoleMenu};
use crate::model::system::sys_role_model::Role;
use crate::model::system::sys_user_model::{
    count_allocated_list, count_unallocated_list, select_allocated_list, select_unallocated_list,
};
use crate::model::system::sys_user_role_model::{
    count_user_role_by_role_id, delete_user_role_by_role_id_user_id, UserRole,
};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_role_vo::*;
use crate::vo::system::sys_user_vo::UserListDataResp;
/*
 *添加角色信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/addRole")]
pub async fn add_sys_role(
    item: web::Json<AddRoleReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("add sys_role params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let name = req.role_name;
    if Role::select_by_role_name(rb, &name).await?.is_some() {
        return Err(AppError::BusinessError("角色名称已存在"));
    }

    let key = req.role_key;
    if Role::select_by_role_key(rb, &key).await?.is_some() {
        return Err(AppError::BusinessError("角色权限已存在"));
    }

    let sys_role = Role {
        id: None,                                //主键
        role_name: name,                         //名称
        role_key: key,                           //角色权限字符串
        data_scope: req.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
        status: req.status,         //状态(1:正常，0:禁用)
        remark: req.remark.unwrap_or_default(), //备注
        del_flag: None,              //删除标志（0代表删除 1代表存在）
        create_time: None,           //创建时间
        update_time: None,           //修改时间
    };

    Role::insert(rb, &sys_role).await?;

    ok_result()
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/deleteRole")]
pub async fn delete_sys_role(
    item: web::Json<DeleteRoleReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("delete sys_role params: {:?}", &item);
    let rb = &data.batis;

    let ids = item.ids.clone();

    if ids.contains(&1) {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    for id in ids {
        if count_user_role_by_role_id(rb, id).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    RoleMenu::delete_by_map(rb, value! {"role_id": &item.ids}).await?;

    RoleDept::delete_by_map(rb, value! {"role_id": &item.ids}).await?;

    Role::delete_by_map(rb, value! {"id": &item.ids}).await?;
    ok_result()
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/updateRole")]
pub async fn update_sys_role(
    item: web::Json<UpdateRoleReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update sys_role params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.id == 1 {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    if Role::select_by_id(rb, &req.id).await?.is_none() {
        return Err(AppError::BusinessError("角色不存在"));
    }

    if let Some(x) = Role::select_by_role_name(rb, &req.role_name).await? {
        if x.id.unwrap_or_default() != req.id {
            return Err(AppError::BusinessError("角色名称已存在"));
        }
    }

    if let Some(x) = Role::select_by_role_key(rb, &req.role_key).await? {
        if x.id.unwrap_or_default() != req.id {
            return Err(AppError::BusinessError("角色权限已存在"));
        }
    }

    let sys_role = Role {
        id: Some(req.id),                       //主键
        role_name: req.role_name,               //名称
        role_key: req.role_key,                 //角色权限字符串
        data_scope: req.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
        status: req.status,         //状态(1:正常，0:禁用)
        remark: req.remark.unwrap_or_default(), //备注
        del_flag: None,              //删除标志（0代表删除 1代表存在）
        create_time: None,           //创建时间
        update_time: None,           //修改时间
    };

    Role::update_by_map(rb, &sys_role, value! {"id": &sys_role.id}).await?;

    ok_result()
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/updateRoleStatus")]
pub async fn update_sys_role_status(
    item: web::Json<UpdateRoleStatusReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update sys_role_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.ids.contains(&1) {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    let update_sql = format!(
        "update sys_role set status = ? where id in ({})",
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
 *查询角色信息详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/queryRoleDetail")]
pub async fn query_sys_role_detail(
    item: web::Json<QueryRoleDetailReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_role_detail params: {:?}", &item);
    let rb = &data.batis;

    match Role::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("角色不存在")),
        Some(x) => {
            let sys_role = QueryRoleDetailResp {
                id: x.id.unwrap_or_default(),               //主键
                role_name: x.role_name,                     //名称
                role_key: x.role_key,                       //角色权限字符串
                data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
                status: x.status,         //状态(1:正常，0:禁用)
                remark: x.remark,         //备注
                del_flag: x.del_flag,     //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            ok_result_data(sys_role)
        }
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/queryRoleList")]
pub async fn query_sys_role_list(
    item: web::Json<QueryRoleListReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_role_list params: {:?}", &item);
    let rb = &data.batis;

    let role_name = item.role_name.as_deref().unwrap_or_default();
    let role_key = item.role_key.as_deref().unwrap_or_default();
    let status = item.status_id.unwrap_or(2);

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = Role::select_sys_role_list(rb, page, role_name, role_key, status).await?;

    let total = d.total;

    let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();
    for x in d.records {
        sys_role_list_data.push(RoleListDataResp {
            id: x.id.unwrap_or_default(),               //主键
            role_name: x.role_name,                     //名称
            role_key: x.role_key,                       //角色权限字符串
            data_scope: x.data_scope, //数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）
            status: x.status,         //状态(1:正常，0:禁用)
            remark: x.remark,         //备注
            del_flag: x.del_flag,     //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    ok_result_page(sys_role_list_data, total)
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/queryRoleMenu")]
pub async fn query_role_menu(
    item: web::Json<QueryRoleMenuReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query role_menu params: {:?}", &item);
    let rb = &data.batis;

    // 查询所有菜单
    let menu_list_all = Menu::select_all(rb).await?;

    let mut menu_list: Vec<MenuDataList> = Vec::new();
    let mut menu_ids: Vec<i64> = Vec::new();

    for y in menu_list_all {
        let x = y.clone();
        menu_list.push(MenuDataList {
            id: x.id.unwrap_or_default(),
            parent_id: x.parent_id,
            title: x.menu_name,
            key: y.id.unwrap_or_default().to_string(),
            label: y.menu_name,
            is_penultimate: y.parent_id == 2,
        });
        menu_ids.push(x.id.unwrap())
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        menu_ids.clear();
        let list = query_menu_by_role(rb, item.role_id).await?;

        for x in list {
            let m_id = x.get("menu_id").unwrap().clone();
            menu_ids.push(m_id)
        }
    }

    ok_result_data(QueryRoleMenuData {
        menu_ids,
        menu_list,
    })
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/updateRoleMenu")]
pub async fn update_role_menu(
    item: web::Json<UpdateRoleMenuReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update role_menu params: {:?}", &item);
    let role_id = item.role_id;

    if role_id == 1 {
        return Err(AppError::BusinessError("不允许操作超级管理员角色"));
    }

    let rb = &data.batis;

    RoleMenu::delete_by_map(rb, value! {"role_id": &role_id}).await?;

    let mut role_menu: Vec<RoleMenu> = Vec::new();

    for id in &item.menu_ids {
        let menu_id = id.clone();
        role_menu.push(RoleMenu {
            id: None,
            create_time: Some(DateTime::now()),
            menu_id,
            role_id: role_id.clone(),
        })
    }

    RoleMenu::insert_batch(rb, &role_menu, item.menu_ids.len() as u64).await?;

    ok_result()
}

/*
 *查询已分配用户角色列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/queryAllocatedList")]
pub async fn query_allocated_list(
    item: web::Json<AllocatedListReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update role_menu params: {:?}", &item);

    let rb = &data.batis;

    let page_no = item.page_no;
    let page_size = item.page_size;
    let role_id = item.role_id;
    let mobile = item.mobile.as_deref().unwrap_or_default();
    let user_name = item.user_name.as_deref().unwrap_or_default();

    let page_no = (page_no - 1) * page_size;
    let d = select_allocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
    for x in d {
        sys_user_list_data.push(UserListDataResp {
            id: x.id.unwrap(),                                  //主键
            mobile: x.mobile,                                   //手机
            user_name: x.user_name,                             //姓名
            nick_name: x.nick_name,                             //用户昵称
            user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
            email: x.email,                                     //用户邮箱
            avatar: x.avatar,                                   //头像路径
            status: x.status,                                   //状态(1:正常，0:禁用)
            dept_id: x.dept_id,                                 //部门ID
            login_ip: x.login_ip,                               //最后登录IP
            login_date: time_to_string(x.login_date),           //最后登录时间
            login_browser: x.login_browser,                     //浏览器类型
            login_os: x.login_os,                               //操作系统
            pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
            remark: x.remark,                                   //备注
            del_flag: x.del_flag,                               //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time),         //创建时间
            update_time: time_to_string(x.update_time),         //修改时间
        })
    }

    let total = count_allocated_list(rb, role_id, user_name, mobile).await?;
    ok_result_page(sys_user_list_data, total)
}

/*
 *查询未分配用户角色列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/queryUnallocatedList")]
pub async fn query_unallocated_list(
    item: web::Json<UnallocatedListReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update role_menu params: {:?}", &item);

    let rb = &data.batis;

    let page_no = item.page_no;
    let page_size = item.page_size;
    let role_id = item.role_id;
    let mobile = item.mobile.as_deref().unwrap_or_default();
    let user_name = item.user_name.as_deref().unwrap_or_default();

    let page_no = (page_no - 1) * page_size;
    let d = select_unallocated_list(rb, role_id, user_name, mobile, page_no, page_size).await?;

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
    for x in d {
        sys_user_list_data.push(UserListDataResp {
            id: x.id.unwrap(),                                  //主键
            mobile: x.mobile,                                   //手机
            user_name: x.user_name,                             //姓名
            nick_name: x.nick_name,                             //用户昵称
            user_type: x.user_type.unwrap_or_default(),         //用户类型（00系统用户）
            email: x.email,                                     //用户邮箱
            avatar: x.avatar,                                   //头像路径
            status: x.status,                                   //状态(1:正常，0:禁用)
            dept_id: x.dept_id,                                 //部门ID
            login_ip: x.login_ip,                               //最后登录IP
            login_date: time_to_string(x.login_date),           //最后登录时间
            login_browser: x.login_browser,                     //浏览器类型
            login_os: x.login_os,                               //操作系统
            pwd_update_date: time_to_string(x.pwd_update_date), //密码最后更新时间
            remark: x.remark,                                   //备注
            del_flag: x.del_flag,                               //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time),         //创建时间
            update_time: time_to_string(x.update_time),         //修改时间
        })
    }

    let total = count_unallocated_list(rb, role_id, user_name, mobile).await?;
    ok_result_page(sys_user_list_data, total)
}

/*
 *取消授权用户
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/cancelAuthUser")]
pub async fn cancel_auth_user(
    item: web::Json<CancelAuthUserReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update role_menu params: {:?}", &item);

    let rb = &data.batis;

    delete_user_role_by_role_id_user_id(rb, item.role_id, item.user_id).await?;

    ok_result()
}

/*
 *批量取消授权用户
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/batchCancelAuthUser")]
pub async fn batch_cancel_auth_user(
    item: web::Json<CancelAuthUserAllReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("cancel auth_user_all params: {:?}", &item);

    let rb = &data.batis;

    let update_sql = format!(
        "delete from sys_user_role where role_id = ? and user_id in ({})",
        item.user_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![value!(item.role_id)];
    param.extend(item.user_ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;

    ok_result()
}

/*
 *批量选择用户授权
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/role/batchAuthUser")]
pub async fn batch_auth_user(
    item: web::Json<SelectAuthUserAllReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("select all_auth_user params: {:?}", &item);
    let role_id = item.role_id;

    let rb = &data.batis;

    let mut user_role: Vec<UserRole> = Vec::new();

    for id in &item.user_ids {
        let user_id = id.clone();
        user_role.push(UserRole {
            id: None,
            create_time: Some(DateTime::now()),
            role_id: role_id.clone(),
            user_id,
        })
    }

    UserRole::insert_batch(rb, &user_role, item.user_ids.len() as u64).await?;

    ok_result()
}
