use super::schema::commenting;
use super::schema::content;
use super::schema::student;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable)]
pub struct Comment {
    content_id: i32,
    commentor_name: String,
    comment: String,
}

#[derive(Deserialize, Serialize, Insertable)]
#[table_name = "commenting"]
pub struct NewComment {
    pub commentor_id: i32,
    pub content_id: i32,
    pub comment_text: String,
}

//after adding comming it will return the vec of all the comments for that content
pub fn add_comment(db_pool: &SqliteConnection, new_comment: NewComment) -> Vec<Comment> {
    diesel::insert_into(commenting::table)
        .values(&new_comment)
        .execute(db_pool)
        .expect("error in inserting comment");

    return display_content_comments(db_pool, new_comment.content_id)
}

//performs an inner join
pub fn display_content_comments(db_pool: &SqliteConnection, content_id: i32)-> Vec<Comment>  {
    commenting::table.inner_join(student::table).inner_join(content::table).select((
            content::id,
            student::name,
            commenting::comment_text,
    ))
        .filter(content::id.eq(content_id))
        .get_results::<Comment>(db_pool).unwrap()
}
