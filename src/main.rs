use sdciith_blogging::db::{student::StudentInfo, content::{all_titles, specific_content}};
use diesel::{Connection, SqliteConnection};

use sdciith_blogging::db::student::{verify_student, add_student};


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

    let data = all_titles(&db_pool);
    println!("{:?}", data);

    let data = specific_content(&db_pool, "bharad", "title4");
    println!("{:?}", data);
}
