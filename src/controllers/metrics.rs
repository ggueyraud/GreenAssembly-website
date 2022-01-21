use crate::{models, utils::ua::UserAgent};
use actix_web::{get, http::HeaderValue, post, web, FromRequest, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use ring::digest;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, PgPool};
use std::str::FromStr;

pub async fn add(
    req: &HttpRequest,
    pool: &PgPool,
    belongs_to: models::metrics::BelongsTo,
) -> Result<Option<Uuid>, actix_web::Error> {
    if let Some(gar_log) = req.headers().get("GAR-LOG") {
        if gar_log == HeaderValue::from_static("false") {
            return Ok(None);
        }
    }

    let ua = UserAgent::from_request(req, &mut actix_web::dev::Payload::None).await?;
    let digest_ip = digest::digest(
        &digest::SHA256,
        req.peer_addr().unwrap().ip().to_string().as_bytes(),
    );
    let digest_ip = format!("{:?}", digest_ip);

    match models::metrics::add(
        &pool,
        belongs_to,
        None,
        &digest_ip,
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
        Err(e) => Err(actix_web::error::ErrorBadRequest(e)),
    }
}

#[derive(Deserialize)]
pub enum BelongsTo {
    Page,
    Project,
}

#[derive(Deserialize)]
pub struct PageInformations {
    path: String,
    sid: String,
    belongs_to: BelongsTo,
}

#[get("/metrics/token")]
pub async fn create(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    infos: web::Query<PageInformations>,
) -> HttpResponse {
    let mut id: Option<i16> = None;
    match infos.belongs_to {
        BelongsTo::Page => {
            #[derive(sqlx::FromRow)]
            struct Page {
                id: i16,
            }

            match models::pages::get::<Page>(&pool, "id", &infos.path).await {
                Ok(page) => id = Some(page.id),
                Err(_) => return HttpResponse::InternalServerError().finish(),
            }
        }
        BelongsTo::Project => match infos.path.split('-').collect::<Vec<_>>().last() {
            Some(post_id) => match post_id.parse::<i16>() {
                Ok(post_id) => match infos.belongs_to {
                    BelongsTo::Project => {
                        if !models::portfolio::projects::exists(&pool, post_id).await {
                            return HttpResponse::NotFound().finish();
                        }

                        id = Some(post_id)
                    }
                    _ => (),
                },
                _ => return HttpResponse::InternalServerError().finish(),
            },
            _ => return HttpResponse::InternalServerError().finish(),
        },
    }

    if let Ok(ua) = UserAgent::from_request(&req, &mut actix_web::dev::Payload::None).await {
        let sid = match Uuid::from_str(infos.sid.as_str()) {
            Ok(val) => val,
            Err(_) => return HttpResponse::BadRequest().finish(),
        };

        let digest_ip = digest::digest(
            &digest::SHA256,
            req.peer_addr().unwrap().ip().to_string().as_bytes(),
        );
        let digest_ip = format!("{:?}", digest_ip);

        if let Ok(id) = models::metrics::add(
            &pool,
            match infos.belongs_to {
                BelongsTo::Project => models::metrics::BelongsTo::Project(id.unwrap()),
                BelongsTo::Page => models::metrics::BelongsTo::Page(id.unwrap()),
            },
            Some(sid),
            &digest_ip,
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
            return HttpResponse::Ok().body(id.to_hyphenated().to_string());
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[derive(Deserialize)]
pub struct Token {
    #[serde(rename(deserialize = "sid"))]
    session_id: Option<String>,
    token: String,
}

#[post("/metrics/log")]
pub async fn log(pool: web::Data<PgPool>, form: web::Form<Token>) -> HttpResponse {
    if let Ok(token) = sqlx::types::Uuid::from_str(&form.token) {
        if models::metrics::exists(&pool, token).await {
            let session_id: Option<Uuid> = match &form.session_id {
                Some(val) => match Uuid::from_str(val.as_str()) {
                    Ok(res) => Some(res),
                    Err(_) => None,
                },
                None => None,
            };

            if let Ok(true) = models::metrics::update_end_date(&pool, session_id, token).await {
                return HttpResponse::Ok().finish();
            }
        }
    }

    HttpResponse::NotFound().finish()
}

// ------------------------------------------------------------------------------ //
// -------------------------------- USER SESSION -------------------------------- //
// ------------------------------------------------------------------------------ //

#[derive(Serialize)]
pub struct SessionData {
    pub sid: String,
    pub vud: DateTime<Utc>,
}

#[get("/metrics/session")]
pub async fn create_session(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let digest_ip = digest::digest(
        &digest::SHA256,
        req.peer_addr().unwrap().ip().to_string().as_bytes(),
    );
    let digest_ip = format!("{:?}", digest_ip);

    if let Ok(session_data) = models::metrics::sessions::add(&pool, &digest_ip).await {
        let sid = session_data.0.to_hyphenated().to_string();
        let vud = session_data.1;
        return HttpResponse::Ok().json(SessionData { sid, vud });
    }

    HttpResponse::InternalServerError().finish()
}
