use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::services;

#[get("")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "blog").await {
        crate::controllers::metrics::add(&req, &pool, page.id).await;

        #[derive(Template)]
        #[template(path = "blog/index.html")]
        struct Index {
            title: String,
            description: Option<String>,
        }

        let page = Index {
            title: page.title,
            description: page.description,
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}