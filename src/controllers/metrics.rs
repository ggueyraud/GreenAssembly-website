use crate::{services::metrics, utils::ua::UserAgent};
use actix_web::FromRequest;
use actix_web::HttpRequest;
use sqlx::PgPool;

pub async fn add(req: &HttpRequest, pool: &PgPool, page_id: i16) -> bool {
    match UserAgent::from_request(req, &mut actix_web::dev::Payload::None).await {
        Ok(ua) => metrics::add(
            &pool,
            page_id,
            req.connection_info().realip_remote_addr(),
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
        .is_ok(),
        _ => false,
    }
}
