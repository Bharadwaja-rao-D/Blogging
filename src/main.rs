#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

use crate::api::index;
use actix_cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{r2d2::ConnectionManager, SqliteConnection};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").expect("database not found");
    let db_pool = Pool::builder()
        .build(ConnectionManager::new(db_url))
        .expect("Error creating a pool");

    let _ = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                web::resource("/api/")
                .route( web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    return Ok(());
}
