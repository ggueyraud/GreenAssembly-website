use crate::controllers::website;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/creation-site-web")
            .service(website::website_creation)
            .service(website::onepage)
            .service(website::showcase)
            .service(website::e_commerce),
    );
}