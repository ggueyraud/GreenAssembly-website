use sqlx::{Error, PgPool};

pub async fn get_cover(pool: &PgPool, project_id: i16) -> Result<String, Error> {
    sqlx::query!(
        r#"SELECT
            f.path AS "path"
        FROM portfolio_project_pictures prp
        JOIN files f ON f.id = prp.file_id
        WHERE prp.project_id = $1 AND prp.order = 0"#,
        project_id
    )
    .fetch_one(pool)
    .await
    .map(|row| row.path)
}

pub async fn get_all(pool: &PgPool, project_id: i16) -> Result<Vec<String>, Error> {
    sqlx::query!(
        "SELECT
            path
        FROM portfolio_project_pictures prp
        JOIN files f ON f.id = prp.file_id
        WHERE project_id = $1
        ORDER BY prp.order",
        project_id
    )
    .fetch_all(pool)
    .await
    .map(|records| {
        records
            .iter()
            .map(|record| record.path.clone())
            .collect::<Vec<_>>()
    })
}
