use crate::controllers::blog;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/blog")
            .service(blog::index)
            .service(blog::show_article)
            .service(blog::show_category)
    );
}
