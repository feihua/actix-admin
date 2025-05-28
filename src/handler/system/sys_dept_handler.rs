use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::{
    check_dept_exist_user, select_children_dept_by_id, select_dept_count,
    select_normal_children_dept_by_id, Dept,
};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dept_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder, Result};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbs::value;

/*
 *添加部门表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/addDept")]
pub async fn add_sys_dept(
    item: web::Json<AddDeptReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("add sys_dept params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    if Dept::select_by_dept_name(rb, &req.dept_name, req.parent_id)
        .await?
        .is_some()
    {
        return BaseResponse::<String>::err_result_msg("部门名称已存在");
    }

    let ancestors = match Dept::select_by_id(rb, &req.parent_id).await? {
        None => return BaseResponse::<String>::err_result_msg("添加失败,上级部门不存在"),
        Some(dept) => {
            if dept.status == 0 {
                return BaseResponse::<String>::err_result_msg("部门停用，不允许添加");
            }
            format!("{},{}", dept.ancestors, &req.parent_id)
        }
    };

    let sys_dept = Dept {
        id: None,                 //部门id
        parent_id: req.parent_id, //父部门id
        ancestors,                //祖级列表
        dept_name: req.dept_name, //部门名称
        sort: req.sort,           //显示顺序
        leader: req.leader,       //负责人
        phone: req.phone,         //联系电话
        email: req.email,         //邮箱
        status: req.status,       //部状态（0：停用，1:正常）
        del_flag: None,           //删除标志（0代表删除 1代表存在）
        create_time: None,        //创建时间
        update_time: None,        //修改时间
    };

    Dept::insert(rb, &sys_dept).await?;

    BaseResponse::<String>::ok_result()
}

/*
 *删除部门表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/deleteDept")]
pub async fn delete_sys_dept(
    item: web::Json<DeleteDeptReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("delete sys_dept params: {:?}", &item);
    let rb = &data.batis;

    if select_dept_count(rb, &item.id).await? > 0 {
        return BaseResponse::<String>::err_result_msg("存在下级部门,不允许删除");
    }

    if check_dept_exist_user(rb, &item.id).await? > 0 {
        return BaseResponse::<String>::err_result_msg("部门存在用户,不允许删除");
    }

    Dept::delete_by_map(rb, value! {"id": &item.id}).await?;

    BaseResponse::<String>::ok_result()
}

/*
 *更新部门表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/updateDept")]
pub async fn update_sys_dept(
    item: web::Json<UpdateDeptReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("update sys_dept params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.parent_id == req.id {
        return BaseResponse::<String>::err_result_msg("上级部门不能是自己");
    }

    let old_ancestors = match Dept::select_by_id(rb, &req.id).await? {
        None => return BaseResponse::<String>::err_result_msg("更新失败,部门不存在"),
        Some(dept) => dept.ancestors,
    };

    let ancestors = match Dept::select_by_id(rb, &req.parent_id).await? {
        None => return BaseResponse::<String>::err_result_msg("更新失败,上级部门不存在"),
        Some(dept) => {
            format!("{},{}", dept.ancestors, &req.parent_id)
        }
    };

    if let Some(x) = Dept::select_by_dept_name(rb, &req.dept_name, req.parent_id).await? {
        if x.id.unwrap_or_default() != req.id {
            return BaseResponse::<String>::err_result_msg("部门名称已存在");
        }
    }

    if select_normal_children_dept_by_id(rb, &req.id).await? > 0 && req.status == 0 {
        return BaseResponse::<String>::err_result_msg("该部门包含未停用的子部门");
    }

    let list = select_children_dept_by_id(rb, &req.id).await?;

    for mut x in list {
        x.ancestors = x
            .ancestors
            .replace(old_ancestors.as_str(), ancestors.as_str());
        Dept::update_by_map(rb, &x,value! {"id": &x.id}).await?;
    }
   

    let sys_dept = Dept {
        id: Some(req.id),             //部门id
        parent_id: req.parent_id,     //父部门id
        ancestors: ancestors.clone(), //祖级列表
        dept_name: req.dept_name,     //部门名称
        sort: req.sort,               //显示顺序
        leader: req.leader,           //负责人
        phone: req.phone,             //联系电话
        email: req.email,             //邮箱
        status: req.status,           //部状态（0：停用，1:正常）
        del_flag: None,               //删除标志（0代表删除 1代表存在）
        create_time: None,            //创建时间
        update_time: None,            //修改时间
    };

    Dept::update_by_map(rb, &sys_dept, value! {"id": &sys_dept.id}).await?;

    if req.status == 1 && sys_dept.ancestors != "0" {
        let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

        let update_sql = format!(
            "update sys_dept set status = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![value!(req.status)];
        param.extend(ids.iter().map(|&id| value!(id)));
        rb.exec(&update_sql, param).await?;
    }
    BaseResponse::<String>::ok_result()
}

/*
 *更新部门表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/updateDeptStatus")]
pub async fn update_sys_dept_status(
    item: web::Json<UpdateDeptStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("update sys_dept_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.status == 1 {
        for id in req.ids.clone() {
            if let Some(x) = Dept::select_by_id(rb, &id).await? {
                let ancestors = x.ancestors;
                let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

                let update_sql = format!(
                    "update sys_dept set status = ? where id in ({})",
                    ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
                );

                let mut param = vec![value!(req.status)];
                param.extend(ids.iter().map(|&id| value!(id)));
                rb.exec(&update_sql, param).await?;
            }
        }

    }
    let update_sql = format!(
        "update sys_dept set status = ? where id in ({})",
        req.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *查询部门表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/queryDeptDetail")]
pub async fn query_sys_dept_detail(
    item: web::Json<QueryDeptDetailReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("query sys_dept_detail params: {:?}", &item);
    let rb = &data.batis;

    match Dept::select_by_id(rb, &item.id).await? {
        None => BaseResponse::<QueryDeptDetailResp>::err_result_data(
            QueryDeptDetailResp::new(),
            "部门不存在",
        ),
        Some(x) => {
            let sys_dept = QueryDeptDetailResp {
                id: x.id.unwrap_or_default(),               //部门id
                parent_id: x.parent_id,                     //父部门id
                ancestors: x.ancestors,                     //祖级列表
                dept_name: x.dept_name,                     //部门名称
                sort: x.sort,                               //显示顺序
                leader: x.leader,                           //负责人
                phone: x.phone,                             //联系电话
                email: x.email,                             //邮箱
                status: x.status,                           //部状态（0：停用，1:正常）
                del_flag: x.del_flag.unwrap_or_default(),   //删除标志（0代表删除 1代表存在）
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDeptDetailResp>::ok_result_data(sys_dept)
        }
    }
}

/*
 *查询部门表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/queryDeptList")]
pub async fn query_sys_dept_list(
    item: web::Json<QueryDeptListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("query sys_dept_list params: {:?}", &item);
    let rb = &data.batis;

    let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let mut list: Vec<DeptListDataResp> = Vec::new();

    for x in Dept::select_page_dept_list(rb, dept_name, status).await? {
        list.push(DeptListDataResp {
            id: x.id.unwrap_or_default(),               //部门id
            parent_id: x.parent_id,                     //父部门id
            ancestors: x.ancestors,                     //祖级列表
            dept_name: x.dept_name,                     //部门名称
            sort: x.sort,                               //显示顺序
            leader: x.leader,                           //负责人
            phone: x.phone,                             //联系电话
            email: x.email,                             //邮箱
            status: x.status,                           //部状态（0：停用，1:正常）
            del_flag: x.del_flag.unwrap_or_default(),   //删除标志（0代表删除 1代表存在）
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    BaseResponse::ok_result_data(list)
}
