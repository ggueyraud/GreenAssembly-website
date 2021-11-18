use sqlx::{PgPool, types::Uuid};

pub async fn add(pool: &PgPool, ip: &str) -> Result<Uuid, sqlx::Error> {
    let res = sqlx::query!(
        "INSERT INTO metric_sessions (ip) VALUES ($1) RETURNING id",
        ip
    )
        .fetch_one(pool)
        .await?;

    Ok(res.id)
}

pub async fn delete(pool: &PgPool, id: Uuid) -> bool {
    sqlx::query!("DELETE FROM metric_sessions WHERE id = $1", id)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected()
        == 1
}