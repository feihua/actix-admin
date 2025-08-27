use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::{count_dict_data_by_type, update_dict_data_type};
use crate::model::system::sys_dict_type_model::DictType;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_type_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbs::value;

/*
 *添加字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/addDictType")]
pub async fn add_sys_dict_type(
    item: web::Json<AddDictTypeReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("add sys_dict_type params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let option = DictType::select_by_dict_type(rb, &req.dict_type).await?;
    if option.is_some() {
        return Err(AppError::BusinessError("字典类型已存在"));
    }

    let sys_dict_type = DictType {
        dict_id: None,                          //字典主键
        dict_name: req.dict_name,               //字典名称
        dict_type: req.dict_type,               //字典类型
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    DictType::insert(rb, &sys_dict_type).await?;

    ok_result()
}

/*
 *删除字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/deleteDictType")]
pub async fn delete_sys_dict_type(
    item: web::Json<DeleteDictTypeReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("delete sys_dict_type params: {:?}", &item);
    let rb = &data.batis;

    let ids = item.ids.clone();
    for id in ids {
        let p = match DictType::select_by_id(rb, &id).await? {
            None => return Err(AppError::BusinessError("字典类型不存在,不能删除")),
            Some(p) => p,
        };

        if count_dict_data_by_type(rb, &p.dict_type).await? > 0 {
            return Err(AppError::BusinessError("已分配,不能删除"));
        }
    }

    DictType::delete_by_map(rb, value! {"id": &item.ids}).await?;

    ok_result()
}

/*
 *更新字典类型表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/updateDictType")]
pub async fn update_sys_dict_type(
    item: web::Json<UpdateDictTypeReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update sys_dict_type params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if DictType::select_by_id(rb, &req.dict_id).await?.is_none() {
        return Err(AppError::BusinessError("字典类型不存在"));
    }

    if let Some(x) = DictType::select_by_dict_type(rb, &req.dict_type).await? {
        if x.dict_id.unwrap_or_default() != req.dict_id {
            return Err(AppError::BusinessError("字典类型已存在"));
        }

        let dict_type = x.dict_type;
        update_dict_data_type(rb, &*req.dict_type, &dict_type).await?;
    }

    let sys_dict_type = DictType {
        dict_id: Some(req.dict_id),             //字典主键
        dict_name: req.dict_name,               //字典名称
        dict_type: req.dict_type,               //字典类型
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    DictType::update_by_map(
        rb,
        &sys_dict_type,
        value! {"dict_id": &sys_dict_type.dict_id},
    )
    .await?;

    ok_result()
}

/*
 *更新字典类型表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/updateDictTypeStatus")]
pub async fn update_sys_dict_type_status(
    item: web::Json<UpdateDictTypeStatusReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("update sys_dict_type_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_dict_type set status = ? where dict_id in ({})",
        req.ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ")
    );

    let mut param = vec![value!(req.status)];
    param.extend(req.ids.iter().map(|&id| value!(id)));
    rb.exec(&update_sql, param).await?;

    ok_result()
}

/*
 *查询字典类型表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/queryDictTypeDetail")]
pub async fn query_sys_dict_type_detail(
    item: web::Json<QueryDictTypeDetailReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_dict_type_detail params: {:?}", &item);
    let rb = &data.batis;

    match DictType::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("字典类型不存在")),
        Some(x) => {
            let sys_dict_type = QueryDictTypeDetailResp {
                dict_id: x.dict_id.unwrap_or_default(),     //字典主键
                dict_name: x.dict_name,                     //字典名称
                dict_type: x.dict_type,                     //字典类型
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            ok_result_data(sys_dict_type)
        }
    }
}

/*
 *查询字典类型表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictType/queryDictTypeList")]
pub async fn query_sys_dict_type_list(
    item: web::Json<QueryDictTypeListReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_dict_type_list params: {:?}", &item);
    let rb = &data.batis;

    let dict_name = item.dict_name.as_deref().unwrap_or_default(); //字典名称
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = DictType::select_dict_type_list(rb, page, dict_name, dict_type, status).await?;

    let mut list: Vec<DictTypeListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(DictTypeListDataResp {
            dict_id: x.dict_id.unwrap_or_default(),     //字典主键
            dict_name: x.dict_name,                     //字典名称
            dict_type: x.dict_type,                     //字典类型
            status: x.status,                           //状态（0：停用，1:正常）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    ok_result_page(list, total)
}
