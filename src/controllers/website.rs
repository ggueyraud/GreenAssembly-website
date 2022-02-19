use crate::models;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;
use std::ops::DerefMut;

#[get("")]
pub async fn website_creation(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) = futures::try_join!(
        models::pages::get(&pool, "/creation-site-web"),
        pool.begin()
    ) {
        if let Ok(token) = crate::controllers::metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
            #[derive(Template)]
            #[template(path = "pages/website_creation.html")]
            struct WebsiteCreation {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
            }

            let page = WebsiteCreation {
                title: page.title,
                description: page.description,
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/onepage")]
pub async fn onepage(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) = futures::try_join!(
        models::pages::get(&pool, "/creation-site-web/onepage"),
        pool.begin()
    ) {
        if let Ok(token) = crate::controllers::metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
            #[derive(Template)]
            #[template(path = "pages/onepage_website.html")]
            struct OnePage {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
            }

            let page = OnePage {
                title: page.title,
                description: page.description,
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/vitrine")]
pub async fn showcase(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) = futures::try_join!(
        models::pages::get(&pool, "/creation-site-web/vitrine"),
        pool.begin()
    ) {
        if let Ok(token) = crate::controllers::metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
            #[derive(Template)]
            #[template(path = "pages/showcase_website.html")]
            struct Showcase {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
            }

            let page = Showcase {
                title: page.title,
                description: page.description,
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/e-commerce")]
pub async fn e_commerce(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) = futures::try_join!(
        models::pages::get(&pool, "/creation-site-web/e-commerce"),
        pool.begin()
    ) {
        if let Ok(token) = crate::controllers::metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
            #[derive(Template)]
            #[template(path = "pages/e_commerce_website.html")]
            struct ECommerce {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
            }

            let page = ECommerce {
                title: page.title,
                description: page.description,
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}
