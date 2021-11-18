use crate::{services::{metrics, pages}, utils::ua::UserAgent};
use actix_web::{FromRequest, web, HttpRequest, HttpResponse, http::HeaderValue, get, post};
use serde::Deserialize;
use std::str::FromStr;
use sqlx::{PgPool, types::Uuid};

pub async fn add(req: &HttpRequest, pool: &PgPool, page_id: i16) -> Result<Option<Uuid>, actix_web::Error> {
    if let Some(gar_log) = req.headers().get("GAR-LOG") {
        if gar_log == HeaderValue::from_static("false") {
            return Ok(None)
        }
    }

    let ua = UserAgent::from_request(req, &mut actix_web::dev::Payload::None).await?;

    match metrics::add(
        &pool,
        page_id,
        &req.peer_addr().unwrap().ip().to_string(),
        ua.product.name.clone(),
        ua.os.name.clone(),
        ua.device.name.clone(),
        match req.headers().get(actix_web::http::header::REFERER) {
            Some(referer) => match referer.to_str() {
                Ok(referer) => Some(referer.to_string()),
                _ => None,
            },
            _ => None,
        },
    )
    .await
    {
        Ok(id) => Ok(Some(id)),
        Err(e) => Err(actix_web::error::ErrorBadRequest(e))
    }

    // match UserAgent::from_request(req, &mut actix_web::dev::Payload::None).await {
    //     Ok(ua) => match metrics::add(
    //         &pool,
    //         page_id,
    //         &req.peer_addr().unwrap().ip().to_string(),
    //         ua.product.name.clone(),
    //         ua.os.name.clone(),
    //         ua.device.name.clone(),
    //         match req.headers().get(actix_web::http::header::REFERER) {
    //             Some(referer) => match referer.to_str() {
    //                 Ok(referer) => Some(referer.to_string()),
    //                 _ => None,
    //             },
    //             _ => None,
    //         },
    //     )
    //     .await
    //     {
    //         Ok(id) => Ok(Some(id)),
    //         Err(e) => Err(actix_web::error::ErrorBadRequest(e))
    //     },
    //     Err(e) => Err(e)
    // }
}

#[derive(Deserialize)]
pub struct PageInformations {
    path: String
}

#[get("/metrics/token")]
pub async fn create(pool: web::Data<PgPool>, req: HttpRequest, infos: web::Query<PageInformations>) -> HttpResponse {
    if let Ok(page_id) = pages::get_id(&pool, &infos.path).await {
        if let Ok(ua) = UserAgent::from_request(&req, &mut actix_web::dev::Payload::None).await {
            if let Ok(id) = metrics::add(
                &pool,
                page_id,
                &req.peer_addr().unwrap().ip().to_string(),
                ua.product.name.clone(),
                ua.os.name.clone(),
                ua.device.name.clone(),
                match req.headers().get(actix_web::http::header::REFERER) {
                    Some(referer) => match referer.to_str() {
                        Ok(referer) => Some(referer.to_string()),
                        _ => None,
                    },
                    _ => None,
                },
            )
            .await {
                return HttpResponse::Ok().body(id.to_hyphenated().to_string())
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[derive(Deserialize)]
pub struct Token {
    token: String
}

#[post("/metrics/log")]
pub async fn log(
    pool: web::Data<PgPool>,
    form: web::Form<Token>
) -> HttpResponse {
    if let Ok(token) = sqlx::types::Uuid::from_str(&form.token) {
        if metrics::exists(&pool, token).await {
            if let Ok(true) = metrics::update_end_date(&pool, token).await {
                return HttpResponse::Ok().finish()
            }
        }
    }

    HttpResponse::NotFound().finish()
}