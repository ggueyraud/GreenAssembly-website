use crate::controllers;
use actix_web::web;

pub mod api;
pub mod admin;
pub mod blog;
pub mod contact;
pub mod newsletter;
pub mod website;
pub mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::index)
        .service(controllers::agency)
        .service(controllers::portfolio)
        .service(controllers::show_project)
        .service(controllers::legals)
        .service(controllers::faq)
        // .service(controllers::sitemap)
        // .service(controllers::robots)
        .service(controllers::metrics::log)
        .service(controllers::metrics::create)
        .service(controllers::metrics::create_session);
}
