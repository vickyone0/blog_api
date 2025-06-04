use diesel::prelude::*;
use crate::schema::posts;
use crate::models::{Post, NewPost, PaginatedPosts, PaginationMeta};

pub fn create_post(conn: &mut PgConnection, new_post: NewPost) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
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

    // Build count query
    let mut count_query = posts::table.into_boxed();
    if let Some(ref search) = search {
        count_query = count_query.filter(
            posts::title.ilike(format!("%{}%", search))
                .or(posts::body.ilike(format!("%{}%", search)))
        );
    }
    let total_docs = count_query
        .count()
        .get_result::<i64>(conn)?;

    // Build records query
    let mut records_query = posts::table.into_boxed();
    if let Some(search) = search {
        records_query = records_query.filter(
            posts::title.ilike(format!("%{}%", search))
                .or(posts::body.ilike(format!("%{}%", search)))
        );
    }
    let records = records_query
        .order(posts::id.desc())
        .limit(per_page)
        .offset(offset)
        .load::<Post>(conn)?;

    let total_pages = (total_docs as f64 / per_page as f64).ceil() as i64;
    let from = offset + 1;
    let to = std::cmp::min(offset + per_page, total_docs);

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