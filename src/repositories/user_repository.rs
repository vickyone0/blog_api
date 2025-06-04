use diesel::prelude::*;
use crate::schema::users;
use crate::models::{User, NewUser};

pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn find_user_by_username(conn: &mut PgConnection, username: &str) -> QueryResult<User> {
    users::table
        .filter(users::username.eq(username))
        .first(conn)
}