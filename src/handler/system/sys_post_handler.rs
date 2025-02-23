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

    if Post::select_by_name(rb, &req.post_name).await?.is_some() {
        return BaseResponse::<String>::err_result_msg("新增岗位失败,岗位名称已存在");
    }

    if Post::select_by_code(rb, &req.post_code).await?.is_some() {
        return BaseResponse::<String>::err_result_msg("新增岗位失败,岗位编码已存在");
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

    Post::insert(rb, &sys_post).await?;

    BaseResponse::<String>::ok_result()
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
        let post_by_id = Post::select_by_id(rb, &id).await?;
        let p = match post_by_id {
            None => {
                return BaseResponse::<String>::err_result_msg("岗位不存在,不能删除");
            }
            Some(p) => p,
        };

        if count_user_post_by_id(rb, id).await? > 0 {
            let msg = format!("{}已分配,不能删除", p.post_name);
            return BaseResponse::<String>::err_result_msg(msg.as_str());
        }
    }

    Post::delete_in_column(rb, "id", &item.ids).await?;

    BaseResponse::<String>::ok_result()
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

    if Post::select_by_id(rb, &req.id).await?.is_none() {
        return BaseResponse::<String>::err_result_msg("更新岗位失败,岗位不存在");
    }

    if let Some(x) = Post::select_by_name(rb, &req.post_name).await? {
        if x.id.unwrap_or_default() != req.id {
            return BaseResponse::<String>::err_result_msg("更新岗位失败,岗位名称已存在");
        }
    }

    if let Some(x) = Post::select_by_code(rb, &req.post_code).await? {
        if x.id.unwrap_or_default() != req.id {
            return BaseResponse::<String>::err_result_msg("更新岗位失败,岗位编码已存在");
        }
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

    Post::update_by_column(rb, &sys_post, "id").await?;

    BaseResponse::<String>::ok_result()
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
    rb.exec(&update_sql, param).await?;

    BaseResponse::<String>::ok_result()
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

    match Post::select_by_id(rb, &item.id).await? {
        None => {
            return BaseResponse::<QueryPostDetailResp>::err_result_data(
                QueryPostDetailResp::new(),
                "岗位不存在",
            );
        }
        Some(x) => {
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
    let d = Post::select_post_list(rb, page, post_code, post_name, status).await?;

    let mut list: Vec<PostListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(PostListDataResp {
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

    BaseResponse::ok_result_page(list, total)
}
