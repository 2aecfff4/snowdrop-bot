pub mod api;
pub mod daily_stats;
pub mod db;
pub mod error;
pub mod migrations;
pub mod models;
pub mod telegram_api;
pub mod total_stats;
pub mod utils;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if cfg!(debug_assertions) {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let bind_port = std::env::var("BIND_PORT").expect("BIND_PORT is not set.");
    let address = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set.");

    let db_pool = web::Data::new(db::initialize_database(&database_url).await);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .wrap(Logger::default())
            .service(crate::api::stats_site::stats_index)
            .service(
                actix_web::web::scope("/api")
                    .service(crate::api::webhook::webhook)
                    .service(crate::api::daily_stats::daily_stats),
            )
    })
    .bind(format!("{}:{}", address, bind_port))?
    .run()
    .await
}
