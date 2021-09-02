use crate::controllers::contact;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contact")
            .service(contact::page)
            .service(contact::send),
    );
}