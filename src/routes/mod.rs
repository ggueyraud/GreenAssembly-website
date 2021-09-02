use crate::controllers;
use actix_web::web;

pub mod website;
pub mod contact;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::index)
    .service(controllers::agency)
    .service(controllers::portfolio)
    .service(controllers::legals)
    .service(controllers::sitemap)
    .service(controllers::robots);
}