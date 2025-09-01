use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::{count_dict_data_by_type, update_dict_data_type};
use crate::model::system::sys_dict_type_model::DictType;
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbatis::rbdc::DateTime;
use rbs::value;

/*
 *添加字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/addDictType")]
pub async fn add_sys_dict_type(item: web::Json<DictTypeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("add sys_dict_type params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    if DictType::select_by_dict_type(rb, &req.dict_type).await?.is_some() {
        return Err(AppError::BusinessError("字典类型已存在"));
    }

    DictType::insert(rb, &DictType::from(req)).await.map(|x| ok_result_data(x.last_insert_id))?
}

/*
 *删除字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/deleteDictType")]
pub async fn delete_sys_dict_type(item: web::Json<DeleteDictTypeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("delete sys_dict_type params: {:?}", &item);
    let rb = &data.batis;

    let ids = &item.ids;

    for id in ids {
        let p = match DictType::select_by_id(rb, &id).await? {
            None => return Err(AppError::BusinessError("字典类型不存在,不能删除")),
            Some(p) => p,
        };

        if count_dict_data_by_type(rb, &p.dict_type).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    DictType::delete_by_map(rb, value! {"id": &item.ids}).await.map(|_| ok_result())?
}

/*
 *更新字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/updateDictType")]
pub async fn update_sys_dict_type(item: web::Json<DictTypeReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_dict_type params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let id = req.id;

    if DictType::select_by_id(rb, &id.unwrap_or_default()).await?.is_none() {
        return Err(AppError::BusinessError("字典类型不存在"));
    }

    if let Some(x) = DictType::select_by_dict_type(rb, &req.dict_type).await? {
        if x.id != id {
            return Err(AppError::BusinessError("字典类型已存在"));
        }

        let dict_type = x.dict_type;
        update_dict_data_type(rb, &*req.dict_type, &dict_type).await?;
    }

    let mut data = DictType::from(req);
    data.update_time = Some(DateTime::now());
    DictType::update_by_map(rb, &data, value! {"id": &id}).await.map(|_| ok_result())?
}

/*
 *更新字典类型表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/updateDictTypeStatus")]
pub async fn update_sys_dict_type_status(item: web::Json<UpdateDictTypeStatusReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("update sys_dict_type_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!("update sys_dict_type set status = ? where id in ({})", req.ids.iter().map(|_| "?").collect::<Vec<&str>>().join(", "));

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await.map(|_| ok_result())?
}

/*
 *查询字典类型表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/queryDictTypeDetail")]
pub async fn query_sys_dict_type_detail(item: web::Json<QueryDictTypeDetailReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_dict_type_detail params: {:?}", &item);
    let rb = &data.batis;

    DictType::select_by_id(rb, &item.id).await?.map_or_else(
        || Err(AppError::BusinessError("字典类型不存在")),
        |x| {
            let data: DictTypeResp = x.into();
            ok_result_data(data)
        },
    )
}

/*
 *查询字典类型表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/queryDictTypeList")]
pub async fn query_sys_dict_type_list(item: web::Json<QueryDictTypeListReq>, data: web::Data<AppState>) -> AppResult<impl Responder> {
    log::info!("query sys_dict_type_list params: {:?}", &item);
    let rb = &data.batis;

    let dict_name = item.dict_name.as_deref().unwrap_or_default(); //字典名称
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    DictType::select_dict_type_list(rb, page, dict_name, dict_type, status)
        .await
        .map(|x| ok_result_page(x.records.into_iter().map(|x| x.into()).collect::<Vec<DictTypeResp>>(), x.total))?
}
