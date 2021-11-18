use serde::Serialize;
use sqlx::{Error, PgPool, types::Uuid};

#[derive(sqlx::FromRow, Serialize)]
pub struct Page {
    pub id: i16,
    pub title: String,
    // pub identifier: String,
    pub description: Option<String>,
    pub is_seo: bool,
    pub is_visible: bool,
}

pub async fn get(pool: &PgPool, path: &str) -> Result<Page, Error> {
    sqlx::query_as!(
        Page,
        "SELECT
            id, title, description, is_seo, is_visible
        FROM pages
        WHERE path = $1
        LIMIT 1",
        path
    )
    .fetch_one(pool)
    .await
}

pub async fn get_id(pool: &PgPool, path: &str) -> Result<i16, Error> {
    let row = sqlx::query!(
        "SELECT
            id
        FROM pages
        WHERE path = $1
        LIMIT 1",
        path
    )
    .fetch_one(pool)
    .await?;
    
    Ok(row.id)
}