use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::{users, posts};
use diesel::sql_types::Array;
use diesel::sql_types::Text;
use diesel::dsl::sql;

use crate::schema::posts_tags;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub created_by: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub created_by: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct PaginatedPosts {
    pub records: Vec<PostWithTags>,
    pub meta: PaginationMeta,
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub current_page: i64,
    pub per_page: i64,
    pub from: i64,
    pub to: i64,
    pub total_pages: i64,
    pub total_docs: i64,
}


#[derive(Queryable, Serialize, Deserialize)]
pub struct PostWithTags {
    #[serde(flatten)]
    pub post: Post,
    pub tags: Vec<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts_tags)]
pub struct PostTag {
    pub post_id: i32,
    pub tag: String,
}