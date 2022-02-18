use crate::models;
use actix_identity::Identity;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use regex::Regex;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    mut form: web::Form<LoginForm>,
    id: Identity,
) -> HttpResponse {
    form.email = form.email.trim().to_string();
    form.password = form.password.trim().to_string();

    let ip = if cfg!(debug_assertions) {
        "localhost".to_string()
    } else {
        req.peer_addr().unwrap().ip().to_string()
    };
    let attempts_counter = models::attempts::count(&pool, &ip, true).await;

    if attempts_counter > 10 {
        return HttpResponse::TooManyRequests().finish();
    }

    models::attempts::add(&pool, &form.email, &ip, true)
        .await
        .unwrap();

    let email_regex = Regex::new(r#"^(([^<>()\[\]\\.,;:\s@"]+(\.[^<>()\[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#).unwrap();

    if !email_regex.is_match(&form.email) {
        return HttpResponse::BadRequest().finish();
    }

    // TODO : move into model
    match models::users::get_password(&pool, &form.email).await {
        Ok(password) => {
            use argon2::{
                password_hash::{PasswordHash, PasswordVerifier},
                Argon2,
            };

            let argon2 = Argon2::default();

            let parsed_hash = PasswordHash::new(&password).unwrap();

            match argon2.verify_password(form.password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    if attempts_counter >= 1 {
                        models::attempts::clear(&pool, &ip).await;
                    }

                    // TODO : see what to save in session
                    // id.remember(serde_json::to_string(&user).unwrap());
                    id.remember(1.to_string());

                    HttpResponse::Ok().json(serde_json::json!({
                        "valid": true
                    }))
                }
                _ => {
                    if attempts_counter + 1 >= 10 {
                        // TODO : inform fail2ban to ban this IP
                    }

                    HttpResponse::Ok().json(serde_json::json!({
                        "valid": false
                    }))
                }
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/logout")]
pub async fn logout(session: Identity) -> HttpResponse {
    session.forget();

    HttpResponse::Found().header("location", "/admin").finish()
}
