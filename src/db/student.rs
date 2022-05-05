use super::schema::student;
use super::schema::student::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

///with outside modules or functions we use StudentInterface to interact and here we might use
///Student to interact

#[derive(Debug, Serialize, Deserialize, Queryable)]
struct Student {
    id: i32,
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "student"]
pub struct StudentInfo<'a> {
    pub name: &'a str,
    pub password: &'a str,
}

///This struct acts as an intermidate
#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct StudentInterface {
    pub id: i32,
    pub name: String,
    ///this will be used to know the actual error
    pub incorrect_reason: String,
}

impl StudentInterface {
    fn new(id_: i32, name_: &str, msg: &str) -> StudentInterface {
        return StudentInterface {
            id: id_,
            name: name_.to_string(),
            incorrect_reason: msg.to_string(),
        };
    }
}

//TODO: Try to use generics so that it would be easy for remaining modules
pub fn add_student(db_pool: &SqliteConnection, new_student: &StudentInfo) -> StudentInterface {
    match search_student(db_pool, new_student.name) {
        Some(_) => {
            return StudentInterface::new(-1, new_student.name, "User exists");
        }
        None => {
            let value = diesel::insert_into(student)
                .values(new_student)
                .execute(db_pool)
                .expect("error inserting");
            return StudentInterface::new(value as i32, new_student.name, "Successful");
        }
    }
}

pub fn verify_student(db_pool: &SqliteConnection, student_info: &StudentInfo) -> StudentInterface {
    match search_student(db_pool, student_info.name) {
        Some(exist) => {
            if exist.password == student_info.password {
                return StudentInterface::new(exist.id, &exist.name, "Successful");
            } else {
                return StudentInterface::new(-1, student_info.name, "Incorrect password");
            }
        }
        None => {
            return StudentInterface::new(-1, student_info.name, "User not found");
        }
    }
}

fn search_student(db_pool: &SqliteConnection, search_user: &str) -> Option<Student> {
    //here we are sure that only one user exists
    match student.filter(name.eq(search_user)).first(db_pool) {
        Ok(found_user) => return Some(found_user),
        Err(_) => return None,
    }
}