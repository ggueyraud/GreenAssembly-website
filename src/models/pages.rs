use sqlx::{Error, PgPool};

// #[derive(sqlx::FromRow, Serialize)]
// pub struct Page {
//     pub id: i16,
//     pub title: String,
//     // pub identifier: String,
//     pub description: Option<String>,
//     pub is_seo: bool,
//     pub is_visible: bool,
// }

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

#[derive(sqlx::FromRow)]
pub struct Page {
    pub id: i16,
    pub title: String
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Page>, Error> {
    let rows = sqlx::query_as!(Page, "SELECT id, title FROM pages WHERE is_visible = TRUE")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}