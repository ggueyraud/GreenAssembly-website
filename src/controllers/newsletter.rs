use crate::models;
use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::ops::DerefMut;

#[derive(Deserialize)]
pub struct SubscribeForm {
    email: String,
}

#[post("/subscribe")]
pub async fn subscribe(pool: web::Data<PgPool>, form: web::Form<SubscribeForm>) -> HttpResponse {
    if models::newsletters::subscribers::exists_for_email(&pool, &form.email).await {
        return HttpResponse::BadRequest().json(json!({ "already_used": true }));
    }

    // Generating token
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    let mut token = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .to_string()
        .into_bytes();
    token.shuffle(&mut rng);
    let token = String::from_utf8(token).unwrap();

    let mut transaction = pool.begin().await.unwrap();

    if models::newsletters::subscribers::add(transaction.deref_mut(), &form.email, &token[..60])
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    // Send email for double opt-in
    // if let Ok(email) = EmailBuilder::new()
    //     .to(form.email.as_str())
    //     .subject(&format!("Confirmation inscription Greenletter"))
    //     .html("Lorem")
    //     .build() {
    //         if let Ok(client) = SmtpClient::new_unencrypted_localhost() {
    //             let mut transport = client.transport();

    //             if transport.send(email.into()).is_ok() {
    transaction.commit().await.unwrap();

    return HttpResponse::Created().finish();
    //             }
    //         }
    //     }

    // HttpResponse::InternalServerError().finish()
}

#[derive(Deserialize)]
pub struct ConfirmSubscriptionForm {
    token: String,
    email: String,
}

#[get("/subscription/confirm")]
pub async fn confirm_subscription(
    pool: web::Data<PgPool>,
    query: web::Query<ConfirmSubscriptionForm>,
) -> HttpResponse {
    #[derive(Template)]
    #[template(path = "pages/newsletter_confirm.html")]
    struct Newsletter {
        exist: bool,
        confirmed: bool,
        already_confirmed: bool,
    }

    let mut page = Newsletter {
        exist: true,
        confirmed: false,
        already_confirmed: false,
    };

    if !models::newsletters::subscribers::exists(&pool, &query.email, &query.token).await {
        page.exist = false;
    } else {
        if models::newsletters::subscribers::is_confirmed(&pool, &query.email).await {
            page.already_confirmed = true;
        } else {
            match models::newsletters::subscribers::confirm(&pool, &query.token, &query.email).await
            {
                Ok(true) => page.confirmed = true,
                _ => page.confirmed = false,
            }
        }
    }

    if let Ok(content) = page.render() {
        return HttpResponse::Ok().content_type("text/html").body(content);
    }

    HttpResponse::InternalServerError().finish()
}

// #[get("/subscription/manage")]
// pub async fn manage(pool: web::Data<PgPool>, query: web::Query<>)

#[derive(serde::Deserialize)]
pub struct Unscribe {
    email: String,
    token: String,
}

#[get("/unscribe")]
pub async fn unscribe(pool: web::Data<PgPool>, query: web::Query<Unscribe>) -> HttpResponse {
    if !models::newsletters::subscribers::exists_for_email(&pool, &query.email).await {
        return HttpResponse::NotFound().finish();
    }

    match models::newsletters::subscribers::delete(&pool, &query.token, &query.email).await {
        Ok(true) => HttpResponse::Ok().finish(),
        Ok(false) => HttpResponse::BadRequest().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
