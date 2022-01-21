use sqlx::PgPool;

pub async fn get_all(pool: &PgPool) -> Vec<super::Category> {
    sqlx::query_as!(
        super::Category,
        r#"SELECT id, name FROM faq_categories ORDER by "order""#
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
