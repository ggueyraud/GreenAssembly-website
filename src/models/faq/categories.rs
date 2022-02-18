use sqlx::{Error, PgPool};

pub async fn get_all(pool: &PgPool) -> Result<Vec<super::Category>, Error> {
    sqlx::query_as!(
        super::Category,
        r#"SELECT id, name FROM faq_categories ORDER by "order""#
    )
    .fetch_all(pool)
    .await
}
