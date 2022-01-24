use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(controllers::users::login)
            .service(controllers::users::logout)
            // .service(controllers::user::lost_password)
            // .service(controllers::user::password_recovery),
    );
}
