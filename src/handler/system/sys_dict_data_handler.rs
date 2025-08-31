use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::DictData;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;

/*
 *添加字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/addDictData")]
pub async fn add_sys_dict_data(item: web::Json<DictDataReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    if DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await?.is_some() {
        return Err(AppError::BusinessError("字典标签已存在"));
    }

    if DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await?.is_some() {
        return Err(AppError::BusinessError("字典键值已存在"));
    }

    DictData::insert(rb, &DictData::from(req)).await.map(|_| ok_result())?
}

/*
 *删除字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/deleteDictData")]
pub async fn delete_sys_dict_data(item: web::Json<DeleteDictDataReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    DictData::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/updateDictData")]
pub async fn update_sys_dict_data(item: web::Json<DictDataReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let id = req.id;

    if DictData::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("字典数据不存在"));
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await? {
        if x.id != id {
            return Err(AppError::BusinessError("字典标签已存在"));
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await? {
        if x.id != id {
            return Err(AppError::BusinessError("字典键值已存在"));
        }
    }

    let mut data = DictData::from(req);
    data.update_time = Some(DateTime::now());
    DictData::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新字典数据表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/updateDictDataStatus")]
pub async fn update_sys_dict_data_status(item: web::Json<UpdateDictDataStatusReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!("update sys_dict_data set status = ? where id in ({})", req.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询字典数据表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/queryDictDataDetail")]
pub async fn query_sys_dict_data_detail(item: web::Json<QueryDictDataDetailReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &data.batis;

    DictData::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("字典数据不存在")),
        |x| {
            let data: DictDataResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询字典数据表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/queryDictDataList")]
pub async fn query_sys_dict_data_list(item: web::Json<QueryDictDataListReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &data.batis;

    let dict_label = item.dict_label.as_deref().unwrap_or_default(); //字典标签
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    DictData::select_dict_data_list(rb, page, dict_label, dict_type, status)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictDataResp>>(), x.total))?
}
