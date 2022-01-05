use sqlx::{Error, PgPool};

pub async fn get_cover(pool: &PgPool, project_id: i16) -> Result<String, Error> {
    let res = sqlx::query!(
        r#"SELECT
            f.path AS "path"
        FROM portfolio_project_pictures prp
        JOIN files f ON f.id = prp.file_id
        WHERE prp.project_id = $1 AND prp.order = 0"#,
        project_id
    )
        .fetch_one(pool)
        .await?;

    Ok(res.path)
}

pub async fn get_all<
    T: std::marker::Unpin + std::marker::Send + for<'c> sqlx::FromRow<'c, sqlx::postgres::PgRow>,
>(
    pool: &PgPool,
    fields: &str,
    project_id: i16
) -> Result<Vec<T>, Error> {
    let pictures = sqlx::query_as::<_, T>(&format!(
        r#"SELECT
            {}
        FROM portfolio_project_pictures prp
        JOIN files f ON f.id = prp.file_id
        WHERE project_id = $1
        ORDER BY prp.order"#,
        fields
    ))
        .bind(project_id)
        .fetch_all(pool)
        .await?;

    Ok(pictures)
}