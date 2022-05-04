use db::student::StudentInfo;
use diesel::{Connection, SqliteConnection};

use crate::db::student::{verify_student, add_student};

#[macro_use]
extern crate diesel;

pub mod db;

fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("database not found");
    let db_pool = SqliteConnection::establish(&db_url).expect("Failed connecting database");
    let student = add_student(&db_pool, &StudentInfo {name: "bharad", password: "dbr"});
    println!("Added {:?}", student);
    let student = verify_student(
        &db_pool,
        &StudentInfo {
            name: "bharad",
            password: "dbr",
        },
    );
    println!("{:?}", student);
}
