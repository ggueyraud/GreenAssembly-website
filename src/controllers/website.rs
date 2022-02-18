use crate::models;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

#[get("")]
pub async fn website_creation(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get(&pool, "/creation-site-web").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

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
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/onepage")]
pub async fn onepage(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get(&pool, "/creation-site-web/onepage").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

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
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/vitrine")]
pub async fn showcase(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get(&pool, "/creation-site-web/vitrine").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

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
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/e-commerce")]
pub async fn e_commerce(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get(&pool, "/creation-site-web/e-commerce").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

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
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}
