//TODO: 1. session stuff
//2. Upvoting stuff

#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

use actix_cors;
use actix_web::{guard, middleware::Logger, web, App, HttpServer};
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

    let host = std::env::var("HOST").expect("database not found");
    let port = std::env::var("PORT").expect("database not found");

    let _frontend = "http://localhost:3000/";

    let _ = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                //returns all the titles, description and author
                //no login required for this
                web::resource("/blog")
                    .route(web::get().to(api::index))
                    //.guard(guard::Header("content-type", "application/json"))
                    //this requires login
                    .route(web::post().to(api::add_blog)),
            )
            .service(
                //returns the title, content and comments of a specific blog
                //no login required for this
                web::resource("/blog/{author}/{title}")
                    .route(web::get().to(api::complete_blog))
                    //.guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(api::add_comment)),
            )
            .service(
                web::resource("/student/signin")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(api::verify_student)),
            )
            .service(
                web::resource("/student/signup")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::post().to(api::add_student)),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await;
    println!("Here");

    return Ok(());
}
