use crate::utils::error_logger::ErrorLogger;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
pub struct AddEmployee {
    pub firstname: String,
    pub lastname: String,
    pub job: String,
    pub quote: Option<String>,
    pub drawing_order: Option<i16>,
}
#[derive(Deserialize)]
pub struct EditEmployee {
    pub id: i32,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub job: Option<String>,
    pub quote: Option<String>,
    pub drawing_order: Option<i16>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Employee {
    pub id: i32,
    pub fullname: Option<String>,
    pub job: String,
    pub quote: Option<String>,
}

pub async fn add_employee(pool: &PgPool, data: &AddEmployee) -> Result<i32, sqlx::Error> {
    let request = sqlx::query!(
        "
        INSERT INTO employees(firstname, lastname, job, quote, drawing_order)
        VALUES($1, $2, $3, $4, $5)
        RETURNING id",
        data.firstname,
        data.lastname,
        data.job,
        data.quote,
        data.drawing_order
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(res) => Ok(res.id),
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn edit_employee(pool: &PgPool, data: &EditEmployee) -> Result<(), sqlx::Error> {
    let request = sqlx::query!(
        "
        UPDATE employees
        SET firstname = COALESCE($2, firstname),
            lastname = COALESCE($3, lastname),
            job = COALESCE($4, job),
            quote = COALESCE($5, quote),
            drawing_order = COALESCE($6, drawing_order)
        WHERE id = $1
        ",
        data.id,
        data.firstname,
        data.lastname,
        data.job,
        data.quote,
        data.drawing_order
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(_) => Ok(()),
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn delete_employee(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    let request = sqlx::query!(
        "
        DELETE FROM employees WHERE id = $1",
        id,
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(_) => Ok(()),
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn get_employees(pool: &PgPool) -> Result<Vec<Employee>, sqlx::Error> {
    let request = sqlx::query_as!(
        Employee,
        "
        SELECT id,
               CONCAT(firstname, ' ', lastname) as fullname,
               job,
               quote
        FROM employees
        ORDER BY drawing_order"
    )
    .fetch_all(pool)
    .await;

    match request {
        Ok(data) => Ok(data),
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}

pub async fn is_employee_exist(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let request = sqlx::query!(
        "
        SELECT EXISTS(SELECT 1 FROM employees WHERE id = $1)",
        id
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(res) => match res.exists {
            Some(val) => Ok(val),
            None => Ok(false),
        },
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
