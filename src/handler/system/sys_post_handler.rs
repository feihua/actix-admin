use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_post_model::Post;
use crate::model::system::sys_user_post_model::count_user_post_by_id;
use crate::vo::system::sys_post_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;

/*
 *添加岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/addPost")]
pub async fn add_sys_post(item: web::Json<PostReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("add sys_post params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    if Post::select_by_name(rb, &req.post_name).await?.is_some() {
        return Err(AppError::BusinessError("岗位名称已存在"));
    }

    if Post::select_by_code(rb, &req.post_code).await?.is_some() {
        return Err(AppError::BusinessError("岗位编码已存在"));
    }

    Post::insert(rb, &Post::from(req)).await.map(|x| ok_result_data(x.last_insert_id))?
}

/*
 *删除岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/deletePost")]
pub async fn delete_sys_post(item: web::Json<DeletePostReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("delete sys_post params: {:?}", &item);
    let rb = &data.batis;

    let ids = item.ids.clone();
    for id in ids {
        if count_user_post_by_id(rb, id).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    Post::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新岗位信息表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/updatePost")]
pub async fn update_sys_post(item: web::Json<PostReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_post params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let id = req.id;

    if Post::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("岗位不存在"));
    }

    if let Some(x) = Post::select_by_name(rb, &req.post_name).await? {
        if x.id != id {
            return Err(AppError::BusinessError("岗位名称已存在"));
        }
    }

    if let Some(x) = Post::select_by_code(rb, &req.post_code).await? {
        if x.id != id {
            return Err(AppError::BusinessError("岗位编码已存在"));
        }
    }

    let mut data = Post::from(req);
    data.update_time = Some(DateTime::now());
    Post::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新岗位信息表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/updatePostStatus")]
pub async fn update_sys_post_status(item: web::Json<UpdatePostStatusReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_post_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!("update sys_post set status = ? where id in ({})", req.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询岗位信息表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/queryPostDetail")]
pub async fn query_sys_post_detail(item: web::Json<QueryPostDetailReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_post_detail params: {:?}", &item);
    let rb = &data.batis;

    Post::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("岗位不存在")),
        |x| {
            let data: PostResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询岗位信息表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/post/queryPostList")]
pub async fn query_sys_post_list(item: web::Json<QueryPostListReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_post_list params: {:?}", &item);
    let rb = &data.batis;

    let post_code = item.post_code.as_deref().unwrap_or_default(); //岗位编码
    let post_name = item.post_name.as_deref().unwrap_or_default(); //岗位名称
    let status = item.status.unwrap_or(2); //部状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    Post::select_post_list(rb, page, post_code, post_name, status)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<PostResp>>(), x.total))?
}
