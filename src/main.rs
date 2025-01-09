#[macro_use]
extern crate rbatis;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;

use crate::model::db::init_db;
use actix_web::{get, middleware as md, web, App, HttpResponse, HttpServer, Responder};
use middleware::auth;
use rbatis::RBatis;
use tracing_actix_web::TracingLogger;
use crate::handler::system::{sys_dept_handler, sys_dict_data_handler, sys_dict_type_handler, sys_login_log_handler, sys_menu_handler, sys_notice_handler, sys_operate_log_handler, sys_post_handler, sys_role_handler, sys_user_handler};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("rust_admin Hello !")
}

// This struct represents state
pub struct AppState {
    pub app_name: String,
    pub batis: RBatis,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log::info!("starting HTTP server at http://0.0.0.0:8788");

    let rb = init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                batis: rb.clone(),
            }))
            .wrap(md::Logger::default())
            .wrap(TracingLogger::default())
            .wrap(auth::Auth)
            .service(index)
            .service(
                (web::scope("/api"))
                    .service(sys_user_handler::add_sys_user)
                    .service(sys_user_handler::delete_sys_user)
                    .service(sys_user_handler::update_sys_user)
                    .service(sys_user_handler::update_sys_user_status)
                    .service(sys_user_handler::update_sys_user_password)
                    .service(sys_user_handler::reset_sys_user_password)
                    .service(sys_user_handler::query_sys_user_detail)
                    .service(sys_user_handler::query_sys_user_list)
                    .service(sys_user_handler::query_user_role)
                    .service(sys_user_handler::update_user_role)
                    .service(sys_user_handler::query_user_menu)
                    
                    .service(sys_user_handler::login)
                    .service(sys_role_handler::add_sys_role)
                    .service(sys_role_handler::delete_sys_role)
                    .service(sys_role_handler::update_sys_role)
                    .service(sys_role_handler::update_sys_role_status)
                    .service(sys_role_handler::query_sys_role_detail)
                    .service(sys_role_handler::query_sys_role_list)
                    .service(sys_role_handler::query_role_menu)
                    .service(sys_role_handler::update_role_menu)
                    .service(sys_role_handler::query_allocated_list)
                    .service(sys_role_handler::query_unallocated_list)
                    .service(sys_role_handler::cancel_auth_user)
                    .service(sys_role_handler::batch_cancel_auth_user)
                    .service(sys_role_handler::batch_auth_user)

                    .service(sys_menu_handler::add_sys_menu)
                    .service(sys_menu_handler::delete_sys_menu)
                    .service(sys_menu_handler::update_sys_menu)
                    .service(sys_menu_handler::update_sys_menu_status)
                    .service(sys_menu_handler::query_sys_menu_detail)
                    .service(sys_menu_handler::query_sys_menu_list)
                    .service(sys_menu_handler::query_sys_menu_list_simple)

                    .service(sys_post_handler::add_sys_post)
                    .service(sys_post_handler::delete_sys_post)
                    .service(sys_post_handler::update_sys_post)
                    .service(sys_post_handler::update_sys_post_status)
                    .service(sys_post_handler::query_sys_post_detail)
                    .service(sys_post_handler::query_sys_post_list)

                    .service(sys_operate_log_handler::delete_sys_operate_log)
                    .service(sys_operate_log_handler::query_sys_operate_log_detail)
                    .service(sys_operate_log_handler::query_sys_operate_log_list)

                    .service(sys_notice_handler::add_sys_notice)
                    .service(sys_notice_handler::delete_sys_notice)
                    .service(sys_notice_handler::update_sys_notice)
                    .service(sys_notice_handler::update_sys_notice_status)
                    .service(sys_notice_handler::query_sys_notice_detail)
                    .service(sys_notice_handler::query_sys_notice_list)

                    .service(sys_login_log_handler::delete_sys_login_log)
                    .service(sys_login_log_handler::query_sys_login_log_detail)
                    .service(sys_login_log_handler::query_sys_login_log_list)

                    .service(sys_dict_type_handler::add_sys_dict_type)
                    .service(sys_dict_type_handler::delete_sys_dict_type)
                    .service(sys_dict_type_handler::update_sys_dict_type)
                    .service(sys_dict_type_handler::update_sys_dict_type_status)
                    .service(sys_dict_type_handler::query_sys_dict_type_detail)
                    .service(sys_dict_type_handler::query_sys_dict_type_list)

                    .service(sys_dict_data_handler::add_sys_dict_data)
                    .service(sys_dict_data_handler::delete_sys_dict_data)
                    .service(sys_dict_data_handler::update_sys_dict_data)
                    .service(sys_dict_data_handler::update_sys_dict_data_status)
                    .service(sys_dict_data_handler::query_sys_dict_data_detail)
                    .service(sys_dict_data_handler::query_sys_dict_data_list)

                    .service(sys_dept_handler::add_sys_dept)
                    .service(sys_dept_handler::delete_sys_dept)
                    .service(sys_dept_handler::update_sys_dept)
                    .service(sys_dept_handler::update_sys_dept_status)
                    .service(sys_dept_handler::query_sys_dept_detail)
                    .service(sys_dept_handler::query_sys_dept_list)
                
                
                ,
                
            )
    })
    .bind(("0.0.0.0", 8788))?
    .run()
    .await
}
