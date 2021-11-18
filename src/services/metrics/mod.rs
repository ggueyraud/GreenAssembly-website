use sqlx::{Error, PgPool, types::Uuid};

pub mod sessions;

pub async fn exists(pool: &PgPool, id: Uuid) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM metrics WHERE id = $1", id
    )
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn add(
    pool: &PgPool,
    page_id: i16,
    ip: &str,
    browser: Option<String>,
    os: Option<String>,
    device_type: Option<String>,
    referer: Option<String>,
) -> Result<Uuid, Error> {
    let res = sqlx::query!(
        "INSERT INTO metrics (page_id, ip, browser, os, device_type, referer)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id",
        page_id,
        ip,
        browser,
        os,
        device_type,
        referer
    )
    .fetch_one(pool)
    .await?;

    Ok(res.id)
}

pub async fn update_end_date(pool: &PgPool, id: Uuid) -> Result<bool, Error> {
    let res = sqlx::query!("UPDATE metrics SET end_date = NOW() WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected() == 1)
}