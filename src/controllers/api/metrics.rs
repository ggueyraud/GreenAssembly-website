use serde::Deserialize;
use chrono::{NaiveDate, Utc};
use actix_web::{HttpResponse, web, get};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct P {
    start: NaiveDate,
    end: NaiveDate
}

#[get("/views-page")]
pub async fn views_page(pool: web::Data<PgPool>, query: web::Query<P>) -> HttpResponse {
    let res = sqlx::query!("SELECT * FROM views_per_page($1, $2)", query.start, query.end)
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    println!("{:?}", res);

    HttpResponse::InternalServerError().finish()
}