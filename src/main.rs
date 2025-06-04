#[macro_use] extern crate rocket;

mod models;
mod schema;
mod repositories;
mod routes;

use rocket_sync_db_pools::database;

#[database("blog_db")]
pub struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/",routes::routes())
}