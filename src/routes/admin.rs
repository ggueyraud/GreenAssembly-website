use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(controllers::admin::dashboard)
            .service(controllers::admin::blog),
    );
}
