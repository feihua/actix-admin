use std::future::{ready, Ready};
use std::rc::Rc;

use crate::common::error::AppError;
use crate::utils::jwt_util::JwtToken;
use actix_web::error;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service: Rc::new(service) }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let path = req.path().to_string();

        let def = header::HeaderValue::from_str("").unwrap();
        let token = req.headers().get("Authorization").unwrap_or(&def).to_str().ok().unwrap().replace("Bearer ", "");

        return Box::pin(async move {
            log::info!("Hi from start. You requested path: {}", path);

            if path.contains("login") {
                let fut = svc.call(req);
                let res = fut.await?;
                return Ok(res);
            }

            if token.len() <= 0 {
                let res = json!({
                    "msg": "token不能为空",
                    "code": 2,
                    "path": path
                });
                return Err(error::ErrorUnauthorized(res.to_string()));
            }

            let jwt_token_e = JwtToken::verify("123", &token);
            let jwt_token = match jwt_token_e {
                Ok(data) => data,
                Err(err) => {
                    let er = match err {
                        AppError::JwtTokenError(s) => s,
                        _ => "no math error".to_string(),
                    };
                    let john = json!({
                        "msg": er,
                        "code": 2,
                        "path": path
                    });
                    log::error!("Hi from start. You requested path: {}, token: {}", path, token);
                    return Err(error::ErrorUnauthorized(john.to_string()));
                }
            };

            let mut flag: bool = false;
            for token_permission in &jwt_token.permissions {
                if token_permission.to_string() == path {
                    flag = true;
                    break;
                }
            }
            req.headers_mut().insert("userId".parse().unwrap(), HeaderValue::from(jwt_token.id));
            if flag {
                let fut = svc.call(req);
                let res = fut.await?;
                Ok(res)
            } else {
                log::error!("Hi from start. You requested path: {:?}", jwt_token.permissions);
                let res = json!({
                    "msg": "无权限访问",
                    "code": 1,
                    "path": path
                });
                Err(error::ErrorUnauthorized(res.to_string()))
            }
        });
    }
}
