use actix_web::web::Json;
use serde::Serialize;
use std::fmt::Debug;
use crate::common::error::{AppResult};

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub success: bool,
    pub data: Option<T>,
}

impl<T> BaseResponse<T>
where
    T: Serialize + Debug + Send,
{
    pub fn ok_result() -> AppResult<Json<BaseResponse<String>>> {
        Ok(Json(BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: None,
        }))
    }

    pub fn ok_result_msg(msg: String) -> AppResult<Json<BaseResponse<String>>> {
        Ok(Json(BaseResponse {
            msg: msg.to_string(),
            code: 0,
            data: None,
        }))
    }

    pub fn ok_result_code(code: i32, msg: String) -> AppResult<Json<BaseResponse<String>>> {
        Ok(Json(BaseResponse {
            msg: msg.to_string(),
            code,
            data: None,
        }))
    }

    pub fn ok_result_data(data: T) -> AppResult<Json<BaseResponse<T>>> {
        Ok(Json(BaseResponse {
            msg: "操作成功".to_string(),
            code: 0,
            data: Some(data),
        }))
    }

    pub fn err_result_data(data: T, msg: String) -> AppResult<Json<BaseResponse<T>>> {
        Ok(Json(BaseResponse {
            msg: msg.to_string(),
            code: 1,
            data: Some(data),
        }))
    }

    pub fn err_result_msg(msg: String) -> AppResult<Json<BaseResponse<String>>> {
        Ok(Json(BaseResponse {
            msg: msg.to_string(),
            code: 1,
            data: None,
        }))
    }

    pub fn err_result_code(code: i32, msg: String) -> AppResult<Json<BaseResponse<String>>> {
        Ok(Json(BaseResponse {
            msg: msg.to_string(),
            code,
            data: None,
        }))
    }
    pub fn ok_result_page(data: T, total: u64) -> AppResult<Json<ResponsePage<T>>> {
        Ok(Json(ResponsePage {
            msg: "操作成功".to_string(),
            code: 0,
            success: true,
            data: Some(data),
            total,
        }))
    }

    pub fn err_result_page(data: T, msg: String) -> AppResult<Json<ResponsePage<T>>> {
        Ok(Json(ResponsePage {
            msg: msg.to_string(),
            code: 1,
            success: false,
            data: Some(data),
            total: 0,
        }))
    }
}
