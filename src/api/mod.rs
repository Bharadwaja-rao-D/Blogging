//contains handlers

use actix_web::{HttpResponse, web};

use crate::db::content;
use crate::Pool;


//at home page we need to display all the titles and descriptions
//this doesnot require logging in
//logging in is required only if a student wants to write a blog or upvote a blog
pub async fn index(db_pool: web::Data<Pool>) -> std::io::Result<HttpResponse>{
    let db_pool = db_pool.get().unwrap();
    let all_contents = content::all_titles(&db_pool);
    return Ok(HttpResponse::Ok().json(all_contents));
}
