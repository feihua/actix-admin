#[macro_use]
extern crate rbatis;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;

use actix_web::{App, get, HttpResponse, HttpServer, middleware, Responder, web};
use rbatis::Rbatis;
use crate::handler::{menu_handler, role_handler, user_handler};
use tracing_actix_web::TracingLogger;
use crate::model::db::init_db;
use crate::utils::auth;

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
    pub batis: Rbatis,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log::info!("starting HTTP server at http://0.0.0.0:8088");

    let rb = init_db().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                batis: rb.clone(),
            }))
            .wrap(middleware::Logger::default())
            .wrap(TracingLogger::default())
            .wrap(auth::Auth)
            .service(index)
            .service((web::scope("/api"))
                .service(user_handler::login)
                .service(user_handler::query_user_menu)
                .service(user_handler::user_list)
                .service(user_handler::user_save)
                .service(user_handler::user_delete)
                .service(user_handler::user_update)
                .service(role_handler::query_role_menu)
                .service(role_handler::update_role_menu)
                .service(role_handler::role_list)
                .service(role_handler::role_save)
                .service(role_handler::role_delete)
                .service(role_handler::role_update)
                .service(menu_handler::menu_list)
                .service(menu_handler::menu_save)
                .service(menu_handler::menu_delete)
                .service(menu_handler::menu_update))
    })
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}