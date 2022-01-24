use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use serde_json::json;
use actix_web::{HttpResponse, web, get};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct P {
    start: NaiveDate,
    end: NaiveDate
}

#[get("/views-page")]
pub async fn views_page(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    #[derive(sqlx::FromRow, Serialize)]
    struct Data {
        date: NaiveDate,
        title: String,
        views: i64
    }

    match sqlx::query_as!(
        Data,
        r#"SELECT "date" AS "date!", title AS "title!", views AS "views!" FROM views_per_page($1, $2)"#,
        query.start,
        query.end
    )
        .fetch_all(pool.as_ref())
        .await {
            Ok(rows) => {
                HttpResponse::Ok().json(rows)
            },
            _ => HttpResponse::InternalServerError().finish()
        }
}