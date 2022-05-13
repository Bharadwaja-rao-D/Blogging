//contains handlers

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::db::commenting::{NewComment, self};
use crate::db::content::{self, specific_content, ContentNew, add_content, get_content_id};
use crate::Pool;
use crate::db::student::{self, StudentInfo};


//at home page we need to display all the titles and descriptions
//this doesnot require logging in
//logging in is required only if a student wants to write a blog or upvote a blog
pub async fn index(db_pool: web::Data<Pool>) -> std::io::Result<HttpResponse>{
    let db_pool = db_pool.get().unwrap();
    let all_contents = content::all_titles(&db_pool);
    return Ok(HttpResponse::Ok().json(all_contents));
}

pub async fn add_student(db_pool: web::Data<Pool>, new_student: web::Json<StudentInfo>) -> std::io::Result<HttpResponse>{
    let db_pool = db_pool.get().unwrap();
    let new_student = new_student.into_inner();
    let info = student::add_student(&db_pool,&new_student);
    return Ok(HttpResponse::Ok().json(info));
}

pub async fn complete_blog(db_pool: web::Data<Pool>, path: web::Path<(String, String)>) -> std::io::Result<HttpResponse>{
    let db_pool = db_pool.get().unwrap();
    let (creator,title) = path.into_inner();
    let full_content = specific_content(&db_pool, &creator, &title);
    return Ok(HttpResponse::Ok().json(full_content));
}

pub async fn add_blog(db_pool: web::Data<Pool>, new_blog: web::Json<ContentNew>) -> std::io::Result<HttpResponse>{
    println!("Here I am");
    let db_pool = db_pool.get().unwrap();
    let new_blog = new_blog.into_inner();
    println!("New blog: {:?}", &new_blog);
    let new_blog  = add_content(&db_pool, new_blog);
    return Ok(HttpResponse::Ok().json(new_blog));
}

#[derive(Deserialize, Serialize)]
pub struct Comment{
    pub commentor_id: i32,
    pub comment: String
}

pub async fn add_comment(db_pool: web::Data<Pool>, new_comment: web::Json<Comment>, path: web::Path<(String, String)>) -> std::io::Result<HttpResponse>{

    let db_pool = db_pool.get().unwrap();
    let new_comment = new_comment.into_inner();
    let (author, title) = path.into_inner();

    let content_id = get_content_id(&db_pool, &author, &title);

    let new_comment = NewComment { commentor_id: new_comment.commentor_id, content_id, comment_text: new_comment.comment };


    return Ok(HttpResponse::Ok().json(commenting::add_comment(&db_pool, new_comment)));
}
