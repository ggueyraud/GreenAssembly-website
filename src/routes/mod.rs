use crate::controllers;
use actix_web::{web, HttpResponse};

pub mod blog;
pub mod contact;
pub mod website;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::index)
        // Redirection old agency route to new route
        .route(
            "/agence",
            web::get().to(|| {
                HttpResponse::MovedPermanently()
                    .header("Location", "/agence-digitale-verte")
                    .finish()
            }),
        )
        .service(controllers::agency)
        .service(controllers::portfolio)
        .service(controllers::legals)
        .service(controllers::faq)
        .service(controllers::sitemap)
        .service(controllers::robots);
}
