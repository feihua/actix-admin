use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_notice_model::Notice;
use crate::vo::system::sys_notice_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;

/*
 *添加通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/addNotice")]
pub async fn add_sys_notice(item: web::Json<NoticeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("add sys_notice params: {:?}", &item);
    let rb = &data.batis;

    let mut req = item.0;

    if Notice::select_by_title(rb, &req.notice_title).await?.is_some() {
        return Err(AppError::BusinessError("公告标题已存在"));
    };

    req.id = None;
    Notice::insert(rb, &Notice::from(req)).await.map(|_| ok_result())?
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/deleteNotice")]
pub async fn delete_sys_notice(item: web::Json<DeleteNoticeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("delete sys_notice params: {:?}", &item);
    let rb = &data.batis;

    Notice::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/updateNotice")]
pub async fn update_sys_notice(item: web::Json<NoticeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_notice params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let id = req.id;
    if Notice::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("通知公告表不存在"));
    }

    if let Some(x) = Notice::select_by_title(rb, &req.notice_title).await? {
        if x.id != id {
            return Err(AppError::BusinessError("公告标题已存在"));
        }
    }

    let mut data = Notice::from(req);
    data.update_time = Some(DateTime::now());
    Notice::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/updateNoticeStatus")]
pub async fn update_sys_notice_status(item: web::Json<UpdateNoticeStatusReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_notice_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!("update sys_notice set status = ? where id in ({})", req.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));

    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询通知公告表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/queryNoticeDetail")]
pub async fn query_sys_notice_detail(item: web::Json<QueryNoticeDetailReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_notice_detail params: {:?}", &item);
    let rb = &data.batis;

    Notice::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("通知公告表不存在")),
        |x| {
            let notice: NoticeResp = x.into();
            ok_result_data(notice)
        },
    )
}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/queryNoticeList")]
pub async fn query_sys_notice_list(item: web::Json<QueryNoticeListReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_notice_list params: {:?}", &item);
    let rb = &data.batis;

    let notice_title = item.notice_title.as_deref().unwrap_or_default();
    let notice_type = item.notice_type.unwrap_or(0); //公告类型（1:通知,2:公告）
    let status = item.status.unwrap_or(2); //公告状态（0:关闭,1:正常 ）

    let page = &PageRequest::new(item.page_no, item.page_size);

    Notice::select_sys_notice_list(rb, page, notice_title, notice_type, status)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<NoticeResp>>(), x.total))?
}
