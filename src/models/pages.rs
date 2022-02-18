use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct PageDetails {
    pub id: i16,
    pub title: String,
    pub description: Option<String>,
}

// #[derive(sqlx::FromRow, Serialize)]
// pub struct Page {
//     pub id: i16,
//     pub title: String,
//     // pub identifier: String,
//     pub description: Option<String>,
//     pub is_seo: bool,
//     pub is_visible: bool,
// }

pub async fn get(pool: &PgPool, identifier: &str) -> Result<PageDetails, Error> {
    sqlx::query_as!(
        PageDetails,
        "SELECT
            id, title, description
        FROM pages
        WHERE path = $1
        LIMIT 1",
        identifier
    )
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
pub struct Page {
    pub id: i16,
    pub title: String,
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Page>, Error> {
    sqlx::query_as!(Page, "SELECT id, title FROM pages WHERE is_visible = TRUE")
        .fetch_all(pool)
        .await
}
