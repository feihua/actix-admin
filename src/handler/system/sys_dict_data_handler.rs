use crate::common::error::{AppError, AppResult};
use crate::common::result::{ok_result, ok_result_data, ok_result_page};
use crate::model::system::sys_dict_data_model::DictData;
use crate::utils::time_util::time_to_string;
use crate::vo::system::sys_dict_data_vo::*;
use crate::AppState;
use actix_web::{post, web, Responder};
use rbatis::plugin::page::PageRequest;
use rbs::value;

/*
 *添加字典数据表
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/addDictData")]
pub async fn add_sys_dict_data(
    item: web::Json<AddDictDataReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("add sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    let req = item.0;

    let option = DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await?;
    if option.is_some() {
        return Err(AppError::BusinessError("新增字典数据失败,字典标签已存在"));
    }

    let detail = DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await?;
    if detail.is_some() {
        return Err(AppError::BusinessError("新增字典数据失败,字典键值已存在"));
    }

    let sys_dict_data = DictData {
        id: None,                        //字典编码
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
    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("delete sys_dict_data params: {:?}", &item);
    let rb = &data.batis;

    DictData::delete_by_map(rb, value! {"id": &item.ids}).await?;
    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("update sys_dict_data params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    if DictData::select_by_id(rb, &req.id).await?.is_none() {
        return Err(AppError::BusinessError("更新字典数据失败,字典数据不存在"));
    }

    if let Some(x) = DictData::select_by_dict_label(rb, &req.dict_type, &req.dict_label).await? {
        if x.id.unwrap_or_default() != req.id {
            return Err(AppError::BusinessError("更新字典数据失败,字典标签已存在"));
        }
    }

    if let Some(x) = DictData::select_by_dict_value(rb, &req.dict_type, &req.dict_value).await? {
        if x.id.unwrap_or_default() != req.id {
            return Err(AppError::BusinessError("更新字典数据失败,字典键值已存在"));
        }
    }

    let sys_dict_data = DictData {
        id: Some(req.id),         //字典编码
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

    DictData::update_by_map(
        rb,
        &sys_dict_data,
        value! {"id": &sys_dict_data.id},
    )
    .await?;
    ok_result()
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
) -> AppResult<impl Responder> {
    log::info!("update sys_dict_data_status params: {:?}", &item);
    let rb = &data.batis;
    let req = item.0;

    let update_sql = format!(
        "update sys_dict_data set status = ? where id in ({})",
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
 *查询字典数据表详情
 *author：刘飞华
 *date：2025/01/08 17:16:44
 */
#[post("/system/dictData/queryDictDataDetail")]
pub async fn query_sys_dict_data_detail(
    item: web::Json<QueryDictDataDetailReq>,
    data: web::Data<AppState>,
) -> AppResult<impl Responder> {
    log::info!("query sys_dict_data_detail params: {:?}", &item);
    let rb = &data.batis;

    match DictData::select_by_id(rb, &item.id).await? {
        None => Err(AppError::BusinessError("字典数据不存在")),
        Some(x) => {
            let sys_dict_data = QueryDictDataDetailResp {
                id: x.id.unwrap_or_default(), //字典编码
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

            ok_result_data(sys_dict_data)
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
) -> AppResult<impl Responder> {
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
            id: x.id.unwrap_or_default(), //字典编码
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

    ok_result_page(list, total)
}
