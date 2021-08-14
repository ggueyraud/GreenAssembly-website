use sqlx::{Error, PgPool};

pub async fn add(
    pool: &PgPool,
    page_id: i16,
    ip: &str,
    browser: Option<String>,
    os: Option<String>,
    device_type: Option<String>,
    referer: Option<String>,
) -> Result<i32, Error> {
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
