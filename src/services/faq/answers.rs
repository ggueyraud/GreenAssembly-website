use sqlx::PgPool;

pub async fn get_all(pool: &PgPool, category_id: i16) -> Vec<super::Answer> {
    sqlx::query_as!(
        super::Answer,
        r#"SELECT id, question, answer FROM faq_answers WHERE category_id = $1 ORDER BY "order""#,
        category_id
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
