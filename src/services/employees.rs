use serde::Serialize;
use sqlx::postgres::PgPool;

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Employee {
    pub id: i32,
    pub fullname: String,
    pub job: String,
    pub description: Option<String>,
    pub picture: String,
}

pub async fn insert(
    pool: &PgPool,
    firstname: &str,
    lastname: &str,
    job: &str,
    description: Option<&str>,
    order: Option<i16>,
) -> Result<i32, sqlx::Error> {
    let res = sqlx::query!(
        "INSERT INTO employees(firstname, lastname, job, description, \"order\")
        VALUES($1, $2, $3, $4, $5)
        RETURNING id",
        firstname,
        lastname,
        job,
        description,
        order
    )
    .fetch_one(pool)
    .await?;

    Ok(res.id)
}

pub async fn update(
    pool: &PgPool,
    id: i32,
    firstname: Option<&str>,
    lastname: Option<&str>,
    job: Option<&str>,
    description: Option<&str>,
    order: Option<i16>,
) -> Result<bool, sqlx::Error> {
    let res = sqlx::query!(
        "UPDATE employees
        SET firstname = COALESCE($2, firstname),
            lastname = COALESCE($3, lastname),
            job = COALESCE($4, job),
            description = COALESCE($5, description),
            \"order\" = COALESCE($6, \"order\")
        WHERE id = $1",
        id,
        firstname,
        lastname,
        job,
        description,
        order
    )
    .execute(pool)
    .await?;

    Ok(res.rows_affected() == 1)
}
pub async fn delete(pool: &PgPool, id: i32) -> bool {
    sqlx::query!("DELETE FROM employees WHERE id = $1", id,)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected()
        == 1
}
pub async fn get_all(pool: &PgPool) -> Result<Vec<Employee>, sqlx::Error> {
    let rows = sqlx::query_as!(
        Employee,
        "SELECT id,
            CONCAT(firstname, ' ', lastname) as \"fullname!\",
            job,
            description,
            picture
        FROM employees
        ORDER BY \"order\""
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn exist(pool: &PgPool, id: i32) -> bool {
    sqlx::query!(
        "SELECT
            1 AS one
        FROM employees
        WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .is_ok()
}
