use std::env;

use actix_web::{get, middleware as md, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{Database, DatabaseConnection};
use tracing_actix_web::TracingLogger;
use handler::system::{menu_handler, role_handler, user_handler};
use crate::middleware::auth;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod middleware;
pub mod common;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .insert_header(("Cache-Control", "no-cache"))
        .body("rust_admin Hello !")
}

// This struct represents state
#[derive(Debug, Clone)]
pub struct AppState {
    pub app_name: String,
    pub conn: DatabaseConnection,
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log::info!("starting HTTP server at http://0.0.0.0:8088");

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(&db_url).await.unwrap();

    let state = AppState { app_name: String::from("Actix Web"), conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(md::Logger::default())
            .wrap(TracingLogger::default())
            .wrap(auth::Auth)
            .service(index)
            .service((web::scope("/api"))
                .service(user_handler::login)
                .service(user_handler::query_user_role)
                .service(user_handler::update_user_role)
                .service(user_handler::query_user_menu)
                .service(user_handler::user_list)
                .service(user_handler::user_save)
                .service(user_handler::user_delete)
                .service(user_handler::user_update)
                .service(user_handler::update_user_password)
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