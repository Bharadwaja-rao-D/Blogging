//contains handlers
//TODO: Need to add web::block for all handlers

use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::db::commenting::{self, NewComment};
use crate::db::content::{self, add_content, get_content_id, specific_content, ContentNew};
use crate::db::student::{self, StudentInfo, StudentInterface};
use crate::Pool;


//at home page we need to display all the titles and descriptions
//this doesnot require logging in
//logging in is required only if a student wants to write a blog or upvote a blog
pub async fn index(db_pool: web::Data<Pool>) -> std::io::Result<HttpResponse> {
    let new_student = StudentInfo {
        name: "anonymous".to_string(),
        password: " ".to_string(),
    };

    let first_run = std::env::var("FIRST_RUN").expect("database not found");
    let db_pool = db_pool.get().unwrap();
    if first_run.parse::<i32>().unwrap() == 1 {

        crate::db::student::add_student(&db_pool, &new_student);
    }
    let all_contents = content::all_titles(&db_pool);
    return Ok(HttpResponse::Ok().json(all_contents));
}

#[derive(Serialize, Deserialize)]
pub struct StudentToken {
    student_info: StudentInterface,
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    uuid: i32,
    exp: usize,
}

impl StudentToken {
    //all the jwt logic will come here
    pub fn generate_token(uuid: i32) -> String {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("valid timestamp")
            .timestamp();
        let header = Header::new(jsonwebtoken::Algorithm::HS512);
        let claims = Claims {
            uuid,
            exp: exp as usize,
        };
        return encode(
            &header,
            &claims,
            &EncodingKey::from_secret(b"thisisthesecret"),
        )
        .unwrap();
    }

    pub fn new(student_info: StudentInterface) -> Self {
        let token = match student_info.id {
            -1 => "null".to_string(),
            _ => StudentToken::generate_token(student_info.id),
        };
        return StudentToken {
            student_info,
            token,
        };
    }
}

pub async fn verify_authentication(auth: BearerAuth) {
    let token = auth.token();
    let _decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(b"thisisthesecret"),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    );
}

//TODO: Set a jwt here
pub async fn add_student(
    db_pool: web::Data<Pool>,
    new_student: web::Json<StudentInfo>,
) -> std::io::Result<HttpResponse> {
    let db_pool = db_pool.get().unwrap();
    let new_student = new_student.into_inner();
    let info = student::add_student(&db_pool, &new_student);
    let info = StudentToken::new(info);
    return Ok(HttpResponse::Ok().json(info));
}

//TODO: Set a jwt here
pub async fn verify_student(
    db_pool: web::Data<Pool>,
    student: web::Json<StudentInfo>,
) -> std::io::Result<HttpResponse> {
    let db_pool = db_pool.get().unwrap();
    let student = student.into_inner();
    let info = student::verify_student(&db_pool, &student);
    let info = StudentToken::new(info);
    return Ok(HttpResponse::Ok().json(info));
}

pub async fn complete_blog(
    db_pool: web::Data<Pool>,
    path: web::Path<(String, String)>,
) -> std::io::Result<HttpResponse> {
    let db_pool = db_pool.get().unwrap();
    let (creator, title) = path.into_inner();
    let full_content = specific_content(&db_pool, &creator, &title);
    return Ok(HttpResponse::Ok().json(full_content));
}

//TODO: Check for jwt, if not present then redirect to signup page
pub async fn add_blog(
    db_pool: web::Data<Pool>,
    new_blog: web::Json<ContentNew>,
) -> std::io::Result<HttpResponse> {
    //need to get access to the header here
    let db_pool = db_pool.get().unwrap();
    let new_blog = new_blog.into_inner();
    println!("New blog: {:?}", &new_blog);
    let new_blog = add_content(&db_pool, new_blog);
    return Ok(HttpResponse::Ok().json(new_blog));
}

#[derive(Deserialize, Serialize)]
pub struct Comment {
    pub commentor_id: i32,
    pub comment: String,
}

//TODO: Check for jwt, if not present then redirect to signup page
pub async fn add_comment(
    db_pool: web::Data<Pool>,
    new_comment: web::Json<Comment>,
    path: web::Path<(String, String)>,
) -> std::io::Result<HttpResponse> {
    let db_pool = db_pool.get().unwrap();
    let new_comment = new_comment.into_inner();
    let (author, title) = path.into_inner();

    let content_id = get_content_id(&db_pool, &author, &title);

    let new_comment = NewComment {
        commentor_id: new_comment.commentor_id,
        content_id,
        comment_text: new_comment.comment,
    };

    return Ok(HttpResponse::Ok().json(commenting::add_comment(&db_pool, new_comment)));
}
