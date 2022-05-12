#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

use actix_cors;
use actix_session::{ SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::{middleware::Logger, web, App, HttpServer, cookie::Key};
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

    let secret_key = Key::generate();
    let redis_conn_string = "127.0.0.1:6379";

    let _ = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(redis_conn_string),
                    secret_key.clone()
                )
                )
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                //returns all the titles, description and author
                //no login required for this
                web::resource("/api/")
                .route( web::get().to(api::index)))
            .service(
                //returns the title, content of a specific blog
                //no login required for this
                web::resource("/api/{author}/{title}")
                .route( web::post().to(api::complete_blog)))
            .service(
                //TODO: This should have logging 
                //upvoting a blog, adding a new blog requires session thing
                web::resource("/api/{new_student}")
                .route( web::post().to(api::add_student)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    return Ok(());
}
