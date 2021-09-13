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

pub async fn get(pool: &PgPool, identifier: &str) -> Result<Page, Error> {
    sqlx::query_as!(
        Page,
        "SELECT
            id, title, description, is_seo, is_visible
        FROM pages
        WHERE identifier = $1
        LIMIT 1",
        identifier
    )
    .fetch_one(pool)
    .await
}
