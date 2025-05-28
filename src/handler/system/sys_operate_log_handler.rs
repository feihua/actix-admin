use actix_web::{post, Responder, Result, web};
use rbatis::plugin::page::PageRequest;
use rbs::value;
use crate::AppState;
use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_operate_log_model::{clean_operate_log, OperateLog};
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_operate_log_vo::*;


/*
 *删除操作日志记录
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/operateLog/deleteOperateLog")]
pub async fn delete_sys_operate_log(item: web::Json<DeleteOperateLogReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("delete sys_operate_log params: {:?}", &item);
    let rb = &data.batis;

    OperateLog::delete_by_map(rb, value! {"id": &item.ids}).await?;
    BaseResponse::<String>::ok_result()
}


/*
 *清空操作日志记录
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/operateLog/cleanOperateLog")]
pub async fn clean_sys_operate_log(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("clean sys_operate_log");
    let rb = &data.batis;

    clean_operate_log(rb).await?;

    BaseResponse::<String>::ok_result()
}

/*
 *查询操作日志记录详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/operateLog/queryOperateLogDetail")]
pub async fn query_sys_operate_log_detail(item: web::Json<QueryOperateLogDetailReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("query sys_operate_log_detail params: {:?}", &item);
    let rb = &data.batis;

    match OperateLog::select_by_id(rb, &item.id).await? {
        None => BaseResponse::<QueryOperateLogDetailResp>::err_result_data(
            QueryOperateLogDetailResp::new(),
            "操作日志不存在",
        ),
        Some(x) => {
            let sys_operate_log = QueryOperateLogDetailResp {
                id: x.id,                                     //日志主键
                title: x.title,                               //模块标题
                business_type: x.business_type,               //业务类型（0其它 1新增 2修改 3删除）
                method: x.method,                             //方法名称
                request_method: x.request_method,             //请求方式
                operator_type: x.operator_type, //操作类别（0其它 1后台用户 2手机端用户）
                operate_name: x.operate_name,   //操作人员
                dept_name: x.dept_name,         //部门名称
                operate_url: x.operate_url,     //请求URL
                operate_ip: x.operate_ip,       //主机地址
                operate_location: x.operate_location, //操作地点
                operate_param: x.operate_param, //请求参数
                json_result: x.json_result,     //返回参数
                status: x.status,               //操作状态(0:异常,正常)
                error_msg: x.error_msg,         //错误消息
                operate_time: time_to_string(x.operate_time), //操作时间
                cost_time: x.cost_time,         //消耗时间
            };

            BaseResponse::<QueryOperateLogDetailResp>::ok_result_data(sys_operate_log)
        }
    }

}

/*
 *查询操作日志记录列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/operateLog/queryOperateLogList")]
pub async fn query_sys_operate_log_list(item: web::Json<QueryOperateLogListReq>, data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    log::info!("query sys_operate_log_list params: {:?}", &item);
    let rb = &data.batis;

    let title = item.title.as_deref().unwrap_or_default(); //模块标题
    let business_type = item.business_type.unwrap_or(4); //业务类型（0其它 1新增 2修改 3删除）
    let method = item.method.as_deref().unwrap_or_default(); //方法名称
    let request_method = item.request_method.as_deref().unwrap_or_default(); //请求方式
    let operator_type = item.operator_type.unwrap_or(3); //操作类别（0其它 1后台用户 2手机端用户）
    let operate_name = item.operate_name.as_deref().unwrap_or_default(); //操作人员
    let dept_name = item.dept_name.as_deref().unwrap_or_default(); //部门名称
    let operate_url = item.operate_url.as_deref().unwrap_or_default(); //请求URL
    let operate_ip = item.operate_ip.as_deref().unwrap_or_default(); //主机地址
    let status = item.status.unwrap_or(2); //操作状态(0:异常,正常)

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = OperateLog::select_page_by_name(
        rb,
        page,
        title,
        &business_type,
        method,
        request_method,
        &operator_type,
        operate_name,
        dept_name,
        operate_url,
        operate_ip,
        &status,
    )
        .await?;

    let mut list: Vec<OperateLogListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(OperateLogListDataResp {
            id: x.id,                                     //日志主键
            title: x.title,                               //模块标题
            business_type: x.business_type,               //业务类型（0其它 1新增 2修改 3删除）
            method: x.method,                             //方法名称
            request_method: x.request_method,             //请求方式
            operator_type: x.operator_type,               //操作类别（0其它 1后台用户 2手机端用户）
            operate_name: x.operate_name,                 //操作人员
            dept_name: x.dept_name,                       //部门名称
            operate_url: x.operate_url,                   //请求URL
            operate_ip: x.operate_ip,                     //主机地址
            operate_location: x.operate_location,         //操作地点
            operate_param: x.operate_param,               //请求参数
            json_result: x.json_result,                   //返回参数
            status: x.status,                             //操作状态(0:异常,正常)
            error_msg: x.error_msg,                       //错误消息
            operate_time: time_to_string(x.operate_time), //操作时间
            cost_time: x.cost_time,                       //消耗时间
        })
    }

    BaseResponse::ok_result_page(list, total)
}
