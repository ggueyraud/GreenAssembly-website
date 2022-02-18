use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Deserialize)]
pub struct P {
    start: NaiveDate,
    end: NaiveDate,
}

#[get("/views-page")]
pub async fn views_page(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    #[derive(FromRow, Serialize)]
    struct Data {
        date: NaiveDate,
        title: String,
        views: i64,
    }

    match sqlx::query_as!(
        Data,
        r#"SELECT "date" AS "date!", title AS "title!", views AS "views!" FROM views_per_page($1, $2)"#,
        query.start,
        query.end
    )
        .fetch_all(pool.as_ref())
        .await {
            Ok(rows) => HttpResponse::Ok().json(rows),
            _ => HttpResponse::InternalServerError().finish()
        }
}

#[get("/devices")]
pub async fn devices(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    #[derive(FromRow, Serialize)]
    struct Data {
        name: String,
        percent: f32,
    }

    match sqlx::query_as!(
        Data,
        r#"SELECT name AS "name!", percent AS "percent!" FROM device_types($1, $2)"#,
        query.start,
        query.end
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/os")]
pub async fn os(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    #[derive(FromRow, Serialize)]
    struct Data {
        name: String,
        percent: f32,
    }

    match sqlx::query_as!(
        Data,
        r#"SELECT name AS "name!", percent AS "percent!" FROM os($1, $2)"#,
        query.start,
        query.end
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/browsers")]
pub async fn browsers(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    #[derive(FromRow, Serialize)]
    struct Data {
        name: String,
        percent: f32,
    }

    match sqlx::query_as!(
        Data,
        r#"SELECT name AS "name!", percent AS "percent!" FROM browsers($1, $2)"#,
        query.start,
        query.end
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
