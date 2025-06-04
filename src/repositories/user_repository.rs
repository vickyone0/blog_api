use diesel::prelude::*;
use crate::schema::users;
use crate::models::{User, NewUser};

pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

