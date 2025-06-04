use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::response::status::Created;
use rocket::http::Status;

use crate::DbConn;
use crate::models::{User, NewUser, Post, NewPost, PaginatedPosts};
use crate::repositories::{user_repository, post_repository};

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

#[post("/posts", data = "<new_post>")]
async fn create_post(
    conn: DbConn,
    new_post: Json<NewPost>,
) -> Result<Created<Json<Post>>, Status> {
    conn.run(|c| {
        post_repository::create_post(c, new_post.into_inner())
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