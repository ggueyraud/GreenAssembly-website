use crate::controllers::newsletter;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/newsletter")
            .service(newsletter::subscribe)
            .service(newsletter::unscribe)
            .service(newsletter::confirm_subscription),
    );
}
