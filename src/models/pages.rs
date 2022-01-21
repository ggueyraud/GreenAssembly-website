use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(sqlx::FromRow, Serialize)]
pub struct Page {
    pub id: i16,
    pub title: String,
    // pub identifier: String,
    pub description: Option<String>,
    pub is_seo: bool,
    pub is_visible: bool,
}

pub async fn get<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
    identifier: &str,
) -> Result<T, Error> {
    let res = sqlx::query_as::<_, T>(&format!(
        "SELECT
            {}
        FROM pages
        WHERE path = $1
        LIMIT 1",
        fields
    ))
    .bind(identifier)
    .fetch_one(pool)
    .await?;

    Ok(res)
}

// pub async fn get(pool: &PgPool, path: &str) -> Result<Page, Error> {
//     sqlx::query_as!(
//         Page,
//         "SELECT
//             id, title, description, is_seo, is_visible
//         FROM pages
//         WHERE path = $1
//         LIMIT 1",
//         path
//     )
//     .fetch_one(pool)
//     .await
// }