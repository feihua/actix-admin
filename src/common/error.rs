use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Failed to complete an HTTP request")]
    // Http { #[from] source: reqwest::Error },
    //
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    //
    // #[error("Failed to update the cache file")]
    // DiskCacheWrite { source: std::io::Error },
    #[error("")]
    JwtTokenError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),
}
pub type AppResult<T> = Result<T, AppError>;

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // let status = match *self {
        //     AppError::NotFound => StatusCode::NOT_FOUND,
        //     AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        // };

        // 返回JSON格式的错误信息
        let body = serde_json::json!({
            "msg": self.to_string(),
            "code": 1,
        });

        HttpResponse::build(StatusCode::OK).json(body)
    }
}