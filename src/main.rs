//TODO: 1. session stuff
//2. Upvoting stuff
//3. Comments part

#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;

use actix_cors;
use actix_web::{middleware::Logger, web, App, HttpServer, guard};
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
    
    let jwt_secret_key = std::env::var("SECRET_KEY").expect("secret key not found");
    let jwt_secret_key = jwt_secret_key.as_bytes();

    // let secret_key = Key::generate();
    // let redis_conn_string = "127.0.0.1:6379";

    let _frontend = "http://localhost:3000/";

    let _ = HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            // .wrap(SessionMiddleware::new(
            //     RedisActorSessionStore::new(redis_conn_string),
            //     secret_key.clone(),
            // ))
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                //returns all the titles, description and author
                //no login required for this
                web::resource("/blog")
                .route(web::get().to(api::index))
                //.guard(guard::Header("content-type", "application/json"))
                //this requires login
                .route(web::post().to(api::add_blog))
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
    //.bind(format!("{}:{}",host,port))?
    .bind("127.0.0.1:8080")?
    .run()
    .await;
    println!("Here");

    return Ok(());
}
