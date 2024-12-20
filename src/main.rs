use std::env;

use crate::middleware::auth;
use actix_web::{get, middleware as md, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use tracing_actix_web::TracingLogger;
use crate::handler::system::{sys_menu_handler, sys_role_handler, sys_user_handler};

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod schema;
pub mod utils;
pub mod vo;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("rust_admin Hello !")
}

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub static RB: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("database_url").expect("database_url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log::info!("starting HTTP server at http://0.0.0.0:8088");

    HttpServer::new(move || {
        App::new()
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
                    .service(sys_user_handler::update_user_password)
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
                    .service(sys_menu_handler::add_sys_menu)
                    .service(sys_menu_handler::delete_sys_menu)
                    .service(sys_menu_handler::update_sys_menu)
                    .service(sys_menu_handler::update_sys_menu_status)
                    .service(sys_menu_handler::query_sys_menu_detail)
                    .service(sys_menu_handler::query_sys_menu_list),
            )
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
