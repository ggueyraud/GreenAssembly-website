use sqlx::{Error, PgPool};

pub async fn get_all<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
) -> Result<Vec<T>, Error> {
    let categories = sqlx::query_as::<_, T>(&format!(
        r#"SELECT
            {}
        FROM portfolio_categories
        ORDER BY "order""#,
        fields
    ))
    .fetch_all(pool)
    .await?;

    Ok(categories)
}
