use sqlx::PgPool;

pub enum Type {
    Category,
    Post,
}

pub async fn exists(pool: &PgPool, r#type: Type, id: i16, uri: &str) -> bool {
    sqlx::query(&format!(
        "SELECT 1 AS one FROM blog_redirections WHERE {} = $1 AND uri = $2 LIMIT 1",
        match r#type {
            Type::Category => "blog_categories",
            Type::Post => "blog_posts",
        }
    ))
    .bind(id)
    .bind(uri)
    .fetch_one(pool)
    .await
    .is_ok()
}
