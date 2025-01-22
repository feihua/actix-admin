use crate::AppState;
use actix_web::{post, web, Responder, Result};
use rbatis::rbatis_codegen::ops::AsProxy;
use rbs::to_value;

use crate::common::result::BaseResponse;
use crate::model::system::sys_dept_model::{
    check_dept_exist_user, select_children_dept_by_id, select_dept_count,
    select_normal_children_dept_by_id, Dept,
};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dept_vo::*;

/*
 *添加部门表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dept/addDept")]
pub async fn add_sys_dept(
    item: web::Json<AddDeptReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("add sys_dept params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let res = Dept::select_by_dept_name(rb, &req.dept_name, req.parent_id).await;
    match res {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("部门名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res = Dept::select_by_id(rb, &req.parent_id).await;
    let ancestors = match res {
        Ok(r) => match r {
            None => {
                return BaseResponse::<String>::err_result_msg(
                    "添加失败,上级部门不存在".to_string(),
                )
            }
            Some(dept) => {
                if dept.status == 0 {
                    return BaseResponse::<String>::err_result_msg(
                        "部门停用，不允许添加".to_string(),
                    );
                }
                format!("{},{}", dept.ancestors, &req.parent_id)
            }
        },
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
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

    let result = Dept::insert(rb, &sys_dept).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
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
) -> Result<impl Responder> {
    log::info!("delete sys_dept params: {:?}", &item);
    let rb = &data.batis;

    let res = select_dept_count(rb, &item.id).await.unwrap_or_default();
    if res > 0 {
        return BaseResponse::<String>::err_result_msg("存在下级部门,不允许删除".to_string());
    }

    let res1 = check_dept_exist_user(rb, &item.id)
        .await
        .unwrap_or_default();
    if res1 > 0 {
        return BaseResponse::<String>::err_result_msg("部门存在用户,不允许删除".to_string());
    }

    let result = Dept::delete_by_column(rb, "id", &item.id).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
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
) -> Result<impl Responder> {
    log::info!("update sys_dept params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.parent_id == req.id {
        return BaseResponse::<String>::err_result_msg("上级部门不能是自己".to_string());
    }

    let res = Dept::select_by_id(rb, &req.id).await;
    let old_ancestors = match res {
        Ok(r) => match r {
            None => {
                return BaseResponse::<String>::err_result_msg("更新失败,部门不存在".to_string())
            }
            Some(dept) => dept.ancestors,
        },
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    };

    let res = Dept::select_by_id(rb, &req.parent_id).await;
    let ancestors = match res {
        Ok(r) => match r {
            None => {
                return BaseResponse::<String>::err_result_msg(
                    "更新失败,上级部门不存在".to_string(),
                )
            }
            Some(dept) => {
                format!("{},{}", dept.ancestors, &req.parent_id)
            }
        },
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    };

    let res = Dept::select_by_dept_name(rb, &req.dept_name, req.parent_id).await;
    match res {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg("部门名称已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let count = select_normal_children_dept_by_id(rb, &req.id)
        .await
        .unwrap_or_default();
    if count > 0 && req.status == 0 {
        return BaseResponse::<String>::err_result_msg("该部门包含未停用的子部门".to_string());
    }

    let res = select_children_dept_by_id(rb, &req.id).await;
    match res {
        Ok(list) => {
            let mut depts = vec![];
            for mut x in list {
                x.ancestors = x
                    .ancestors
                    .replace(old_ancestors.as_str(), ancestors.as_str());
                depts.push(x)
            }
            let result = Dept::update_by_column_batch(rb, &depts, "id", depts.len() as u64).await;
            if result.is_err() {
                return BaseResponse::<String>::err_result_msg(
                    "修改下级部门祖级列失败".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
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

    let result = Dept::update_by_column(rb, &sys_dept, "id").await;

    if result.is_err() {
        return BaseResponse::<String>::err_result_msg("更新部门失败".to_string());
    }

    if req.status == 1 && sys_dept.ancestors != "0" {
        let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

        let update_sql = format!(
            "update sys_dept set status = ? where id in ({})",
            ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
        );

        let mut param = vec![to_value!(req.status)];
        param.extend(ids.iter().map(|&id| to_value!(id)));
        let res = rb.exec(&update_sql, param).await;

        match res {
            Ok(_u) => BaseResponse::<String>::ok_result(),
            Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
        }
    } else {
        BaseResponse::<String>::ok_result()
    }
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
) -> Result<impl Responder> {
    log::info!("update sys_dept_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if req.status == 1 {
        for id in req.ids.clone() {
            let result = Dept::select_by_id(rb, &id).await.unwrap_or_default();
            if result.is_some() {
                let ancestors = result.unwrap().ancestors;
                let ids = ancestors.split(",").map(|s| s.i64()).collect::<Vec<i64>>();

                let update_sql = format!(
                    "update sys_dept set status = ? where id in ({})",
                    ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", ")
                );

                let mut param = vec![to_value!(req.status)];
                param.extend(ids.iter().map(|&id| to_value!(id)));
                let res_dept = rb.exec(&update_sql, param).await;

                if res_dept.is_err() {
                    return BaseResponse::<String>::err_result_msg(
                        "更新上级部门状态异常".to_string(),
                    );
                }
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

    let mut param = vec![to_value!(req.status)];
    param.extend(req.ids.iter().map(|&id| to_value!(id)));
    let result = rb.exec(&update_sql, param).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
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
) -> Result<impl Responder> {
    log::info!("query sys_dept_detail params: {:?}", &item);
    let rb = &data.batis;

    let result = Dept::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_dept) => {
            if opt_sys_dept.is_none() {
                return BaseResponse::<QueryDeptDetailResp>::err_result_data(
                    QueryDeptDetailResp::new(),
                    "部门表不存在".to_string(),
                );
            }
            let x = opt_sys_dept.unwrap();

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
        Err(err) => BaseResponse::<QueryDeptDetailResp>::err_result_data(
            QueryDeptDetailResp::new(),
            err.to_string(),
        ),
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
) -> Result<impl Responder> {
    log::info!("query sys_dept_list params: {:?}", &item);
    let rb = &data.batis;

    let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let result = Dept::select_page_dept_list(rb, dept_name, status).await;

    let mut sys_dept_list_data: Vec<DeptListDataResp> = Vec::new();

    match result {
        Ok(d) => {
            for x in d {
                let sys_dept = DeptListDataResp {
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
                sys_dept_list_data.push(sys_dept);
            }

            BaseResponse::<Vec<DeptListDataResp>>::ok_result_data(sys_dept_list_data)
        }
        Err(err) => BaseResponse::<Vec<DeptListDataResp>>::err_result_data(
            DeptListDataResp::new(),
            err.to_string(),
        ),
    }
}
