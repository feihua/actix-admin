use actix_web::{post, Responder, Result, web};
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use crate::AppState;
use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_post_model::{ Post };
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_post_vo::*;

/*
 *添加岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/addPost")]
pub async fn add_sys_post(item: web::Json<AddPostReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("add sys_post params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let res_by_name = Post::select_by_name(rb, &req.post_name).await;
    match res_by_name {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    "新增岗位失败,岗位名称已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_code = Post::select_by_code(rb, &req.post_code).await;
    match res_by_code {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg(
                    "新增岗位失败,岗位编码已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_post = Post {
        id: None,                                //岗位id
        post_code: req.post_code,               //岗位编码
        post_name: req.post_name,               //岗位名称
        sort: req.sort,                         //显示顺序
        status: req.status,                     //部状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    let result = Post::insert(rb, &sys_post).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/deletePost")]
pub async fn delete_sys_post(item: web::Json<DeletePostReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("delete sys_post params: {:?}", &item);
    let rb = &data.batis;

    let ids = item.ids.clone();
    for id in ids {
        let post_by_id = Post::select_by_id(rb, &id).await;
        let p = match post_by_id {
            Ok(p) => {
                if p.is_none() {
                    return BaseResponse::<String>::err_result_msg(
                        "岗位不存在,不能删除".to_string(),
                    );
                } else {
                    p.unwrap()
                }
            }
            Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
        };

        let res = count_user_post_by_id(rb, id).await;
        if res.unwrap_or_default() > 0 {
            let msg = format!("{}已分配,不能删除", p.post_name);
            return BaseResponse::<String>::err_result_msg(msg);
        }
    }

    let result = Post::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/updatePost")]
pub async fn update_sys_post(item: web::Json<UpdatePostReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("update sys_post params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;


    let result = Post::select_by_id(rb, &req.id).await;

    match result {
        Ok(d) => {
            if d.is_none() {
                return BaseResponse::<String>::err_result_msg(
                    "更新岗位失败,岗位不存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_name = Post::select_by_name(rb, &req.post_name).await;
    match res_by_name {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg(
                    "更新岗位失败,岗位名称已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let res_by_code = Post::select_by_code(rb, &req.post_code).await;
    match res_by_code {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg(
                    "更新岗位失败,岗位编码已存在".to_string(),
                );
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_post = Post {
        id: Some(req.id),                       //岗位id
        post_code: req.post_code,               //岗位编码
        post_name: req.post_name,               //岗位名称
        sort: req.sort,                         //显示顺序
        status: req.status,                     //部状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //更新时间
    };

    let result = Post::update_by_column(rb, &sys_post, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/updatePostStatus")]
pub async fn update_sys_post_status(item: web::Json<UpdatePostStatusReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("update sys_post_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_post set status = ? where id in ({})",
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
 *查询岗位信息表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/queryPostDetail")]
pub async fn query_sys_post_detail(item: web::Json<QueryPostDetailReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("query sys_post_detail params: {:?}", &item);
    let rb = &data.batis;

    let result = Post::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_post) => {
            if opt_sys_post.is_none() {
                return BaseResponse::<QueryPostDetailResp>::err_result_data(
                    QueryPostDetailResp::new(),
                    "岗位信息表不存在".to_string(),
                );
            }
            let x = opt_sys_post.unwrap();
            
            let sys_post = QueryPostDetailResp {
                id: x.id.unwrap_or_default(),               //岗位id
                post_code: x.post_code,                     //岗位编码
                post_name: x.post_name,                     //岗位名称
                sort: x.sort,                               //显示顺序
                status: x.status,                           //部状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //更新时间
            };

            BaseResponse::<QueryPostDetailResp>::ok_result_data(sys_post)
        }
        Err(err) => {
            BaseResponse::<QueryPostDetailResp>::err_result_data(QueryPostDetailResp::new(), err.to_string())
        }
    }

}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/queryPostList")]
pub async fn query_sys_post_list(item: web::Json<QueryPostListReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("query sys_post_list params: {:?}", &item);
    let rb = &data.batis;

    let post_code = item.post_code.as_deref().unwrap_or_default(); //岗位编码
    let post_name = item.post_name.as_deref().unwrap_or_default(); //岗位名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Post::select_post_list(rb, page, post_code, post_name, status).await;

    let mut sys_post_list_data: Vec<PostListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                sys_post_list_data.push(PostListDataResp {
                    id: x.id.unwrap_or_default(),               //岗位id
                    post_code: x.post_code,                     //岗位编码
                    post_name: x.post_name,                     //岗位名称
                    sort: x.sort,                               //显示顺序
                    status: x.status,                           //部状态（0：停用，1:正常）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //更新时间
                })
            }

            BaseResponse::ok_result_page(sys_post_list_data, total)
        }
        Err(err) => BaseResponse::err_result_page(PostListDataResp::new(), err.to_string()),
    }

}
