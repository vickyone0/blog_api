// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        created_by -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(posts -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
