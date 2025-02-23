use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_dict_data_model::DictData;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder, Result};
use rbatis::plugin::page::PageRequest;
use rbs::to_value;

/*
 *添加字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/addDictData")]
pub async fn add_sys_dict_data(
    item: web::Json<AddDictDataReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    if DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label)
        .await?
        .is_some()
    {
        return BaseResponse::<String>::err_result_msg("新增字典数据失败,字典标签已存在");
    }

    if DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value)
        .await?
        .is_some()
    {
        return BaseResponse::<String>::err_result_msg("新增字典数据失败,字典键值已存在");
    }

    let sys_dict_data = DictData {
        dict_code: None,                        //字典编码
        dict_sort: req.dict_sort,               //字典排序
        dict_label: req.dict_label,             //字典标签
        dict_value: req.dict_value,             //字典键值
        dict_type: req.dict_type,               //字典类型
        css_class: req.css_class,               //样式属性（其他样式扩展）
        list_class: req.list_class,             //格回显样式
        is_default: req.is_default,             //是否默认（Y是 N否）
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    DictData::insert(rb, &sys_dict_data).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *删除字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/deleteDictData")]
pub async fn delete_sys_dict_data(
    item: web::Json<DeleteDictDataReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    DictData::delete_in_column(rb, "dict_code", &item.ids).await?;
    BaseResponse::<String>::ok_result()
}

/*
 *更新字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/updateDictData")]
pub async fn update_sys_dict_data(
    item: web::Json<UpdateDictDataReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if DictData::select_by_id(rb, &req.dict_code).await?.is_none() {
        return BaseResponse::<String>::err_result_msg("更新字典数据失败,字典数据不存在");
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await? {
        if x.dict_code.unwrap_or_default() != req.dict_code {
            return BaseResponse::<String>::err_result_msg("更新字典数据失败,字典标签已存在");
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await? {
        if x.dict_code.unwrap_or_default() != req.dict_code {
            return BaseResponse::<String>::err_result_msg("更新字典数据失败,字典键值已存在");
        }
    }

    let sys_dict_data = DictData {
        dict_code: Some(req.dict_code),         //字典编码
        dict_sort: req.dict_sort,               //字典排序
        dict_label: req.dict_label,             //字典标签
        dict_value: req.dict_value,             //字典键值
        dict_type: req.dict_type,               //字典类型
        css_class: req.css_class,               //样式属性（其他样式扩展）
        list_class: req.list_class,             //格回显样式
        is_default: req.is_default,             //是否默认（Y是 N否）
        status: req.status,                     //状态（0：停用，1:正常）
        remark: req.remark.unwrap_or_default(), //备注
        create_time: None,                      //创建时间
        update_time: None,                      //修改时间
    };

    DictData::update_by_column(rb, &sys_dict_data, "dict_code").await?;
    BaseResponse::<String>::ok_result()
}

/*
 *更新字典数据表状态
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/updateDictDataStatus")]
pub async fn update_sys_dict_data_status(
    item: web::Json<UpdateDictDataStatusReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_dict_data set status = ? where dict_code in ({})",
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
 *查询字典数据表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/queryDictDataDetail")]
pub async fn query_sys_dict_data_detail(
    item: web::Json<QueryDictDataDetailReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &data.batis;

    match DictData::select_by_id(rb, &item.id).await? {
        None => BaseResponse::<QueryDictDataDetailResp>::err_result_data(
            QueryDictDataDetailResp::new(),
            "字典数据不存在",
        ),
        Some(x) => {
            let sys_dict_data = QueryDictDataDetailResp {
                dict_code: x.dict_code.unwrap_or_default(), //字典编码
                dict_sort: x.dict_sort,                     //字典排序
                dict_label: x.dict_label,                   //字典标签
                dict_value: x.dict_value,                   //字典键值
                dict_type: x.dict_type,                     //字典类型
                css_class: x.css_class,                     //样式属性（其他样式扩展）
                list_class: x.list_class,                   //格回显样式
                is_default: x.is_default,                   //是否默认（Y是 N否）
                status: x.status,                           //状态（0：停用，1:正常）
                remark: x.remark,                           //备注
                create_time: time_to_string(x.create_time), //创建时间
                update_time: time_to_string(x.update_time), //修改时间
            };

            BaseResponse::<QueryDictDataDetailResp>::ok_result_data(sys_dict_data)
        }
    }
}

/*
 *查询字典数据表列表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/queryDictDataList")]
pub async fn query_sys_dict_data_list(
    item: web::Json<QueryDictDataListReq>,
    data: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    log::info!("query sys_dict_data_list params: {:?}", &item);
    let rb = &data.batis;

    let dict_label = item.dict_label.as_deref().unwrap_or_default(); //字典标签
    let dict_type = item.dict_type.as_deref().unwrap_or_default(); //字典类型
    let status = item.status.unwrap_or(2); //状态（0：停用，1:正常）

    let page = &PageRequest::new(item.page_no, item.page_size);
    let d = DictData::select_dict_data_list(rb, page, dict_label, dict_type, status).await?;

    let mut list: Vec<DictDataListDataResp> = Vec::new();

    let total = d.total;

    for x in d.records {
        list.push(DictDataListDataResp {
            dict_code: x.dict_code.unwrap_or_default(), //字典编码
            dict_sort: x.dict_sort,                     //字典排序
            dict_label: x.dict_label,                   //字典标签
            dict_value: x.dict_value,                   //字典键值
            dict_type: x.dict_type,                     //字典类型
            css_class: x.css_class,                     //样式属性（其他样式扩展）
            list_class: x.list_class,                   //格回显样式
            is_default: x.is_default,                   //是否默认（Y是 N否）
            status: x.status,                           //状态（0：停用，1:正常）
            remark: x.remark,                           //备注
            create_time: time_to_string(x.create_time), //创建时间
            update_time: time_to_string(x.update_time), //修改时间
        })
    }

    BaseResponse::ok_result_page(list, total)
}
