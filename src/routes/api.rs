use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/metrics")
                    .service(controllers::api::metrics::views_page)
                    .service(controllers::api::metrics::devices)
                    .service(controllers::api::metrics::os)
                    .service(controllers::api::metrics::browsers)
            )
    );
}
