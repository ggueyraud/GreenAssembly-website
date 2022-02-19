use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct PageDetails {
    pub id: i16,
    pub title: String,
    pub description: Option<String>,
}

pub async fn get_id_from_path(pool: &PgPool, path: &str) -> Result<i16, Error> {
    sqlx::query!("SELECT id FROM pages WHERE path = $1 LIMIT 1", path)
        .fetch_one(pool)
        .await
        .map(|row| row.id)
}

pub async fn get(pool: &PgPool, path: &str) -> Result<PageDetails, Error> {
    sqlx::query_as!(
        PageDetails,
        "SELECT
            id, title, description
        FROM pages
        WHERE path = $1
        LIMIT 1",
        path
    )
    .fetch_one(pool)
    .await
}

#[derive(FromRow)]
pub struct Page {
    pub id: i16,
    pub title: String,
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Page>, Error> {
    sqlx::query_as!(Page, "SELECT id, title FROM pages WHERE is_visible = TRUE")
        .fetch_all(pool)
        .await
}
