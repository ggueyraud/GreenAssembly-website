use crate::controllers;
use actix_web::web;

pub mod blog;
pub mod contact;
pub mod website;
pub mod newsletter;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(controllers::index)
        // Redirection old agency route to new route
        // .route(
        //     "/agence",
        //     web::get().to(|| {
        //         HttpResponse::MovedPermanently()
        //             .header("Location", "/agence-digitale-verte")
        //             .finish()
        //     }),
        // )
        .service(controllers::agency)
        .service(controllers::portfolio)
        .service(controllers::show_project)
        .service(controllers::legals)
        .service(controllers::faq)
        .service(controllers::sitemap)
        .service(controllers::robots)
        .service(controllers::metrics::log)
        .service(controllers::metrics::create)
        .service(controllers::metrics::create_session);
}
