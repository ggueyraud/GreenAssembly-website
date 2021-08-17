use actix_web::{HttpRequest, HttpResponse};

pub async fn is_banned(req: &HttpRequest, pool: &PgPool) -> HttpResponse {
    if crate::services::ips_banned::get(
        &self.pool,
        &req.peer_addr().unwrap().ip().to_string()
    ).await {
        HttpResponse::Forbidden()
            .content_type("text/plain; charset=utf-8")
            .body(format!("Votre adresse IP a été détecté comme intrusive, l'accès au site-web vous est interdis jusqu'au {}", ip_banned.date.format("%d/%m/%Y à %H:%M")))
            .finish()
    } else {

    }
}