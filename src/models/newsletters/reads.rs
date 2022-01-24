use sqlx::{types::Uuid, Error, PgPool};

pub async fn add(pool: &PgPool, newsletter_id: i16, subscriber_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO newsletter_reads (newsletter_id, subscriber_id) VALUES ($1, $2)",
        newsletter_id,
        subscriber_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
