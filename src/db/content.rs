use super::schema::{content, student};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

///with outside modules or functions we use StudentInterface to interact and here we might use
///Student to interact

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ContentJist {
    pub content_id: i32,
    pub title: String,
    pub description: String,
    pub creator_name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
//TODO: Add comments parts
pub struct Content {
    pub content_id: i32,
    pub title: String,
    pub description: String,
    pub body: String,
    pub creator_name: String,
    pub upvotes: i32,
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="content"]
pub struct ContentNew{
    pub title: String,
    pub description: String,
    pub body: String,
    pub creator_id: i32,
}

pub fn all_titles(db_pool: &SqliteConnection) -> Vec<ContentJist> {
    return content::table
        .inner_join(student::table)
        .select((
            content::id,
            content::title,
            content::description,
            student::name,
        ))
        .get_results::<ContentJist>(db_pool)
        .unwrap();
}

pub fn specific_content(db_pool: &SqliteConnection, creator_name: &str, title: &str) -> Vec<Content>{
    return content::table
        .inner_join(student::table)
        .select((
                content::id,
                content::title,
                content::description,
                content::body,
                student::name,
                content::upvotes,
        ))
        .filter(student::name.eq(creator_name).and(content::title.eq(title)))
        .get_results::<Content>(db_pool).unwrap();
}

pub fn add_content(db_pool: &SqliteConnection, new_content: ContentNew) -> Content {
}
