use sqlx::{FromRow, PgPool};

#[derive(FromRow, Clone)]
pub struct Employee {
    pub fullname: String,
    pub job: String,
    pub description: Option<String>,
    pub picture: String,
}

pub async fn get_employees(pool: &PgPool) -> Vec<Employee> {
    sqlx::query_as!(
        Employee,
        r#"SELECT
            CONCAT(firstname, ' ', lastname) AS "fullname!",
            job,
            description,
            f.path AS "picture"
        FROM users u
        JOIN files f ON u.picture_id = f.id
        WHERE is_employed = TRUE
        ORDER BY "order""#
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
