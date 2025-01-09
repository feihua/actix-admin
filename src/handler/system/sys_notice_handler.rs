use actix_web::{post, Responder, Result, web};
use rbatis::plugin::page::PageRequest;
use rbs::to_value;
use crate::AppState;

use crate::common::result::BaseResponse;
use crate::model::system::sys_notice_model::{ Notice };
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_notice_vo::*;

/*
 *添加通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/addNotice")]
pub async fn add_sys_notice(item: web::Json<AddNoticeReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("add sys_notice params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let res = Notice::select_by_title(rb, &req.notice_title).await;
    match res {
        Ok(r) => {
            if r.is_some() {
                return BaseResponse::<String>::err_result_msg("公告标题已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_notice = Notice {
        id: None,                                //公告ID
        notice_title: req.notice_title,         //公告标题
        notice_type: req.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: req.notice_content,     //公告内容
        status: req.status,                     //公告状态（0:关闭,1:正常 ）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = Notice::insert(rb, &sys_notice).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/deleteNotice")]
pub async fn delete_sys_notice(item: web::Json<DeleteNoticeReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("delete sys_notice params: {:?}", &item);
    let rb = &data.batis;

    let result = Notice::delete_in_column(rb, "id", &item.ids).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/updateNotice")]
pub async fn update_sys_notice(item: web::Json<UpdateNoticeReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update sys_notice params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let res = Notice::select_by_title(rb, &req.notice_title).await;

    match res {
        Ok(r) => {
            if r.is_some() && r.unwrap().id.unwrap_or_default() != req.id {
                return BaseResponse::<String>::err_result_msg("公告标题已存在".to_string());
            }
        }
        Err(err) => return BaseResponse::<String>::err_result_msg(err.to_string()),
    }

    let sys_notice = Notice {
        id: Some(req.id),                       //公告ID
        notice_title: req.notice_title,         //公告标题
        notice_type: req.notice_type,           //公告类型（1:通知,2:公告）
        notice_content: req.notice_content,     //公告内容
        status: req.status,                     //公告状态（0:关闭,1:正常 ）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                       //创建时间
        update_time: None,                       //修改时间
    };

    let result = Notice::update_by_column(rb, &sys_notice, "id").await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新通知公告表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/updateNoticeStatus")]
pub async fn update_sys_notice_status(item: web::Json<UpdateNoticeStatusReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("update sys_notice_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_notice set status = ? where id in ({})",
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
 *查询通知公告表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/queryNoticeDetail")]
pub async fn query_sys_notice_detail(item: web::Json<QueryNoticeDetailReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query sys_notice_detail params: {:?}", &item);
    let rb = &data.batis;

    let result = Notice::select_by_id(rb, &item.id).await;

    match result {
        Ok(opt_sys_notice) => {
            if opt_sys_notice.is_none() {
                return BaseResponse::<QueryNoticeDetailResp>::err_result_data(
                    QueryNoticeDetailResp::new(),
                    "通知公告表不存在".to_string(),
                );
            }
            let x = opt_sys_notice.unwrap();
            
            let sys_notice = QueryNoticeDetailResp {
                id: x.id.unwrap_or_default(),               //公告ID
                notice_title: x.notice_title,               //公告标题
                notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
                notice_content: x.notice_content,           //公告内容
                status: x.status,                           //公告状态（0:关闭,1:正常 ）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryNoticeDetailResp>::ok_result_data(sys_notice)
        }
        Err(err) => {
            BaseResponse::<QueryNoticeDetailResp>::err_result_data(QueryNoticeDetailResp::new(), err.to_string())
        }
    }

}

/*
 *查询通知公告表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/notice/queryNoticeList")]
pub async fn query_sys_notice_list(item: web::Json<QueryNoticeListReq>, data: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("query sys_notice_list params: {:?}", &item);
    let rb = &data.batis;

    let notice_title = item.notice_title.as_deref().unwrap_or_default();
    let notice_type = item.notice_type.unwrap_or(0); //公告类型（1:通知,2:公告）
    let status = item.status.unwrap_or(2); //公告状态（0:关闭,1:正常 ）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let result = Notice::select_sys_notice_list(rb, page, notice_title, notice_type, status).await;

    let mut data: Vec<NoticeListDataResp> = Vec::new();
    match result {
        Ok(d) => {
            let total = d.total;

            for x in d.records {
                data.push(NoticeListDataResp {
                    id: x.id.unwrap_or_default(),               //公告ID
                    notice_title: x.notice_title,               //公告标题
                    notice_type: x.notice_type,                 //公告类型（1:通知,2:公告）
                    notice_content: x.notice_content,           //公告内容
                    status: x.status,                           //公告状态（0:关闭,1:正常 ）
                    remark: x.remark,                           //备注
                    create_time: time_to_string(x.create_time), //创建时间
                    update_time: time_to_string(x.update_time), //修改时间
                })
            }

            BaseResponse::ok_result_page(data, total)
        }
        Err(err) => BaseResponse::err_result_page(NoticeListDataResp::new(), err.to_string()),
    }

}
