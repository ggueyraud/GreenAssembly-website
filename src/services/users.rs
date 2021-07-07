use crate::utils::error_logger::ErrorLogger;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
pub struct AddUser {
    pub login: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct EditUser {
    pub id: i32,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub login: String,
}

pub async fn add_user(pool: &PgPool, data: &AddUser) -> Result<i32, sqlx::Error> {
    let request = sqlx::query!(
        "
        INSERT INTO users(login, password)
        VALUES($1, $2)
        RETURNING id",
        data.login,
        data.password,
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(res) => Ok(res.id),
        Err(e) => {
            ErrorLogger::write(
                "users_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn edit_user(pool: &PgPool, data: &EditUser) -> Result<(), sqlx::Error> {
    let request = sqlx::query!(
        "
        UPDATE users
        SET login = COALESCE($2, login),
            password = COALESCE($3, password)
        WHERE id = $1
        ",
        data.id,
        data.login,
        data.password
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(_) => Ok(()),
        Err(e) => {
            ErrorLogger::write(
                "users_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    let request = sqlx::query!(
        "
        DELETE FROM users WHERE id = $1",
        id,
    )
    .fetch_one(pool)
    .await;

    match request {
        Ok(_) => Ok(()),
        Err(e) => {
            ErrorLogger::write(
                "users_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let request = sqlx::query_as!(
        User,
        "
        SELECT id,
               login
        FROM users
        ORDER BY id"
    )
    .fetch_all(pool)
    .await;

    match request {
        Ok(data) => Ok(data),
        Err(e) => {
            ErrorLogger::write(
                "users_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}

pub async fn is_user_exist(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let request = sqlx::query!(
        "
        SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
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
                "users_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );
            Err(e)
        }
    }
}
