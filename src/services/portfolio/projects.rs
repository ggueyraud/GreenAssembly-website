use sqlx::{Error, PgPool};

pub async fn exists(pool: &PgPool, id: i16) -> bool {
    sqlx::query!("SELECT 1 AS one FROM portfolio_projects WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn get_all<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
) -> Result<Vec<T>, Error> {
    let projects = sqlx::query_as::<_, T>(&format!(
        "SELECT
            {}
        FROM portfolio_projects
        ORDER BY id DESC",
        fields
    ))
        .fetch_all(pool)
        .await?;

    Ok(projects)
}

pub async fn get<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
    id: i16,
) -> Result<T, Error> {
    let query = format!("SELECT {} FROM portfolio_projects WHERE id = $1 LIMIT 1", fields);

    let res = sqlx::query_as::<_, T>(&query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(res)
}