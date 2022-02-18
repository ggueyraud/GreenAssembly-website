use super::Category;
use sqlx::{Error, PgPool};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Category>, Error> {
    sqlx::query_as!(
        Category,
        r#"SELECT
            id, name
        FROM portfolio_categories
        ORDER BY "order""#
    )
    .fetch_all(pool)
    .await
}
