use sqlx::{Error, FromRow, PgPool};

#[derive(FromRow)]
pub struct CategoryInformations {
    pub name: String,
    pub description: Option<String>,
}

pub async fn get(pool: &PgPool, id: i16) -> Result<CategoryInformations, Error> {
    sqlx::query_as!(
        CategoryInformations,
        "SELECT name, description FROM blog_categories WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

#[derive(FromRow)]
pub struct Category {
    pub name: String,
    pub uri: String,
}

pub async fn get_all_visible(pool: &PgPool) -> Result<Vec<Category>, Error> {
    sqlx::query_as!(
        Category,
        r#"SELECT name, uri FROM blog_categories WHERE is_visible = TRUE ORDER BY "order""#
    )
    .fetch_all(pool)
    .await
}

#[derive(FromRow, serde::Serialize)]
pub struct CategoryAdminInformations {
    pub id: i16,
    pub name: String,
}

pub async fn _get_all(pool: &PgPool) -> Result<Vec<CategoryAdminInformations>, Error> {
    sqlx::query_as!(
        CategoryAdminInformations,
        r#"SELECT id, name FROM blog_categories ORDER BY "order""#
    )
    .fetch_all(pool)
    .await
}

pub async fn exists(pool: &PgPool, id: i16, name: &str) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM blog_categories WHERE id = $1 AND name = $2",
        id,
        name
    )
    .fetch_one(pool)
    .await
    .is_ok()
}