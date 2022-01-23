use sqlx::{PgPool, Error, types::Uuid};

pub async fn exists(pool: &PgPool, email: &str, token: &str) -> bool {
    sqlx::query!(
        "SELECT 1 AS one FROM newsletter_subscribers WHERE email = $1 AND token = $2",
        email,
        token
    )
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn exists_for_email(pool: &PgPool, email: &str) -> bool {
    sqlx::query!("SELECT 1 AS one FROM newsletter_subscribers WHERE email = $1", email)
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn is_confirmed(pool: &PgPool, email: &str) -> bool {
    sqlx::query!(
        "SELECT
            1 AS one
        FROM newsletter_subscribers
        WHERE email = $1 AND is_confirmed = TRUE",
        email
    )
        .fetch_one(pool)
        .await
        .is_ok()
}

pub async fn add(
    pool: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    email: &str,
    token: &str
) -> Result<Uuid, Error> {
    let id = sqlx::query!(
        "INSERT INTO newsletter_subscribers
            (email, token)
        VALUES ($1, $2)
        RETURNING id",
        email,
        token
    )
        .fetch_one(pool)
        .await?
        .id;

    Ok(id)
}

pub async fn confirm(pool: &PgPool, token: &str, email: &str) -> Result<bool, Error> {
    let rows = sqlx::query!(
        "UPDATE newsletter_subscribers
        SET is_confirmed = TRUE
        WHERE token = $1 AND email = $2",
        token,
        email
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows == 1)
}

pub async fn delete(pool: &PgPool, token: &str, email: &str) -> Result<bool, Error> {
    let rows = sqlx::query!(
        "UPDATE newsletter_subscribers SET unscribe_date = NOW() WHERE token = $1 AND email = $2",
        token,
        email
    )
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows == 1)
}