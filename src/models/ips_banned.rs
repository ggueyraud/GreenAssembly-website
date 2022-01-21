use sqlx::{Error, PgPool};

pub async fn add(pool: &PgPool, ip: &str) -> Result<i32, Error> {
    let res = sqlx::query!(
        "INSERT INTO ips_banned (ip)
        VALUES ($1)
        RETURNING id",
        ip
    )
    .fetch_one(pool)
    .await?;

    Ok(res.id)
}

#[derive(sqlx::FromRow, Debug)]
pub struct IPBanned {
    pub id: i32,
    pub ip: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

pub async fn get(pool: &PgPool, ip: &str) -> Result<IPBanned, Error> {
    let row = sqlx::query!(
        "SELECT
            id, date
        FROM ips_banned
        WHERE ip = $1
        AND date BETWEEN NOW() - INTERVAL '10 minutes' AND NOW()
        LIMIT 1",
        ip
    )
    .fetch_one(pool)
    .await?;

    Ok(IPBanned {
        id: row.id,
        ip: ip.to_string(),
        date: row.date,
    })
}

pub async fn count(pool: &PgPool, ip: &str) -> i64 {
    sqlx::query!("SELECT COUNT(id) FROM ips_banned WHERE ip = $1", ip)
        .fetch_one(pool)
        .await
        .unwrap()
        .count
        .unwrap()
}

pub async fn is_banned(pool: &PgPool, ip: &str) -> bool {
    sqlx::query!(
        "SELECT
            1 AS one
        FROM ips_banned
        WHERE ip = $1 AND
        date BETWEEN NOW() -INTERVAL '1 hour' AND NOW()",
        ip
    )
    .fetch_one(pool)
    .await
    .is_ok()
}
