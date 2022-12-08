use super::commenting::{display_content_comments, Comment};
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
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
#[table_name = "content"]
pub struct ContentNew {
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

#[derive(Serialize)]
pub struct WholeContentInfo {
    pub content: Content,
    comments: Vec<Comment>,
}

pub fn get_content_id(db_pool: &SqliteConnection, creator_name: &str, title: &str) -> i32 {
    let content: i32 = content::table
        .inner_join(student::table)
        .select(content::id)
        .filter(student::name.eq(creator_name).and(content::title.eq(title)))
        .first(db_pool)
        .unwrap();

    return content;
}

//TODO: Here we need to show all the comments also
pub fn specific_content(
    db_pool: &SqliteConnection,
    creator_name: &str,
    title: &str,
) -> WholeContentInfo {
    let content: Vec<Content> = content::table
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
        .get_results::<Content>(db_pool)
        .unwrap();

    //Here there will be only one blog for sure
    let content = content.get(0).unwrap().clone();
    let content_id = content.content_id;

    let comments = display_content_comments(db_pool, content_id);

    return WholeContentInfo { content, comments };
}

pub fn add_content(db_pool: &SqliteConnection, new_content: ContentNew) -> WholeContentInfo {
    //search for the student

    let creator_name: String = student::table
        .filter(student::id.eq(&new_content.creator_id))
        .select(student::name)
        .first(db_pool)
        .expect("Not found");

    let _info = diesel::insert_into(content::table)
        .values(&new_content)
        .execute(db_pool)
        .expect("Error inserting a student");

    return specific_content(db_pool, &creator_name, &new_content.title);
}
/*

//to show changes we return Content
pub fn up_vote(db_pool: &SqliteConnection, blog_id: i32) -> Content{
}
*/
