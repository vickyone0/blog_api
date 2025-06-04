use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::response::status::Created;
use rocket::http::Status;
use rocket::serde::Deserialize;
use crate::DbConn;
use crate::models::{User, NewUser, NewPost, PaginatedPosts};
use crate::repositories::{user_repository, post_repository};
use crate::models::PostWithTags;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user,
        create_post,
        list_posts,
    ]
}

#[post("/users", data = "<new_user>")]
async fn create_user(
    conn: DbConn,
    new_user: Json<NewUser>,
) -> Result<Created<Json<User>>, Status> {
    conn.run(|c| {
        user_repository::create_user(c, new_user.into_inner())
            .map(|u| Created::new("/users").body(Json(u)))
            .map_err(|_| Status::InternalServerError)
    }).await
}

#[derive(Deserialize, Clone)]
pub struct NewPostWithTags {
    #[serde(flatten)]
    pub post: NewPost,
    pub tags: Vec<String>,
}

#[post("/posts", data = "<new_post>")]
async fn create_post(
    conn: DbConn,
    new_post: Json<NewPostWithTags>,
) -> Result<Created<Json<PostWithTags>>, Status> {
    conn.run(move |c| {
        post_repository::create_post_with_tags(
             c,
             new_post.post.clone(),
            new_post.tags.clone(),
        )
            .map(|p| Created::new("/posts").body(Json(p)))
            .map_err(|_| Status::InternalServerError)
    }).await
}


#[get("/posts?<page>&<per_page>&<search>")]
async fn list_posts(
    conn: DbConn,
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
) -> Result<Json<PaginatedPosts>, Status> {
    conn.run(move |c| {
        post_repository::list_posts(c, page, per_page, search)
            .map(Json)
            .map_err(|_| Status::InternalServerError)
    }).await
}