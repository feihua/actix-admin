use std::env;

use actix_web::{App, get, HttpResponse, HttpServer, middleware, Responder, web};
use diesel::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use tracing_actix_web::TracingLogger;

use crate::handler::{menu_handler, role_handler, user_handler};
use crate::utils::auth;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod schema;

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
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
});

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();
    log::info!("starting HTTP server at http://0.0.0.0:8088");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
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