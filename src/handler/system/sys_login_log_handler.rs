use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_login_log_model::{clean_login_log, LoginLog};
use crate::vo::system::sys_login_log_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbs::value;

/*
 *删除系统访问记录
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/loginLog/deleteLoginLog")]
pub async fn delete_sys_login_log(
    item: web::Json<DeleteLoginLogReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("delete sys_login_log params: {:?}", &item);
    let rb = &data.batis;

    LoginLog::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *清空系统登录日志
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/loginLog/cleanLoginLog")]
pub async fn clean_sys_login_log(data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("clean sys_login_log ");
    let rb = &data.batis;

    clean_login_log(rb).await.map(|_| ok_result())?
}

/*
 *查询系统访问记录详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/loginLog/queryLoginLogDetail")]
pub async fn query_sys_login_log_detail(
    item: web::Json<QueryLoginLogDetailReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_login_log_detail params: {:?}", &item);
    let rb = &data.batis;

    LoginLog::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("系统访问记录不存在")),
        |x| {
            let data: LoginLogResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询系统访问记录列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/loginLog/queryLoginLogList")]
pub async fn query_sys_login_log_list(
    item: web::Json<QueryLoginLogListReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_login_log_list params: {:?}", &item);
    let rb = &data.batis;

    let name = item.login_name.as_deref().unwrap_or_default(); //登录账号
    let ipaddr = item.ipaddr.as_deref().unwrap_or_default(); //登录IP地址
    let browser = item.browser.as_deref().unwrap_or_default(); //浏览器类型
    let os = item.os.as_deref().unwrap_or_default(); //操作系统
    let status = item.status.unwrap_or(2); //登录状态(0:失败,1:成功)

    let page = &PageRequest::new(item.page_no, item.page_size);
    LoginLog::select_login_log_list(rb, page, name, ipaddr, browser, os, &status).await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<LoginLogResp>>(), x.total))?

}
