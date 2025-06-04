use diesel::prelude::*;
use crate::schema::posts;
use crate::models::{Post,PostWithTags,PostTag, NewPost, PaginatedPosts, PaginationMeta,PostWithUserAndTags};
use diesel::dsl::sql;
use diesel::pg::sql_types::Array;
use diesel::sql_types::Text;
use crate::schema::posts_tags;
use crate::schema::users;
use crate::models::UserInfo;




pub fn create_post_with_tags(
    conn: &mut PgConnection,
    new_post: NewPost,
    tags: Vec<String>,
) -> QueryResult<PostWithTags> {
    conn.transaction(|conn| {
        // Insert the post first
        let post: Post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)?;

        // Insert tags if any
        if !tags.is_empty() {
    let post_tags = tags.into_iter()
        .map(|tag_str| PostTag {
            post_id: post.id,
            tag: tag_str,
        })
        .collect::<Vec<_>>();

    diesel::insert_into(posts_tags::table)
        .values(&post_tags)
        .execute(conn)?;
}

        let tags_vec = get_tags_for_post(conn, post.id)?;

// Return post with tags
Ok(PostWithTags {
    post,
    tags: tags_vec,
})
    })
}

pub fn get_tags_for_post(conn: &mut PgConnection, post_id_value: i32) -> QueryResult<Vec<String>> {
    posts_tags::table
        .filter(posts_tags::post_id.eq(post_id_value))
        .select(posts_tags::tag)
        .load(conn)
}

pub fn list_posts(
    conn: &mut PgConnection,
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
) -> QueryResult<PaginatedPosts> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;
    let mut count_query = posts::table.left_join(users::table).into_boxed();
let mut records_query = posts::table.left_join(users::table).into_boxed();


    if let Some(search) = search {
        count_query = count_query.filter(
            posts::title.ilike(format!("%{}%", search))
                .or(posts::body.ilike(format!("%{}%", search)))
                .or(users::username.ilike(format!("%{}%", search)))
        );
    }

    let total_docs = count_query
        .count()
        .get_result::<i64>(conn)?;

    let total_pages = (total_docs as f64 / per_page as f64).ceil() as i64;
    let from = offset + 1;
    let to = std::cmp::min(offset + per_page, total_docs);

    // Main query with joins and aggregations
    let records = records_query
        .order(posts::id.desc())
        .limit(per_page)
        .offset(offset)
        .select((
            posts::all_columns,
            // User info as nullable
            users::id.nullable(),
            users::username.nullable(),
            users::first_name.nullable(),
            users::last_name.nullable(),
            // Tags array
            sql::<Array<Text>>("ARRAY(SELECT tag FROM posts_tags WHERE post_id = posts.id)")
        ))
        .load::<(Post, Option<i32>, Option<String>, Option<String>, Option<String>, Vec<String>)>(conn)?
        .into_iter()
        .map(|(post, user_id, username, first_name, last_name, tags)| {
            PostWithUserAndTags {
                post,
                created_by: match user_id {
                    Some(id) => Some(UserInfo {
                        user_id: id,
                        username: username.unwrap(),
                        first_name: first_name.unwrap(),
                        last_name,
                    }),
                    None => None,
                },
                tags,
            }
        })
        .collect();

    Ok(PaginatedPosts {
        records,
        meta: PaginationMeta {
            current_page: page,
            per_page,
            from,
            to,
            total_pages,
            total_docs,
        },
    })
}
