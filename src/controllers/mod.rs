use crate::services;
use crate::templates::Employee;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

use sqlx::PgPool;
pub mod contact;
pub mod employees;
pub mod metrics;
pub mod users;
pub mod website;

#[get("/")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "accueil").await {
        crate::controllers::metrics::add(&req, &pool, page.id).await;

        #[derive(Template)]
        #[template(path = "index.html")]
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

#[get("/agence")]
async fn agency(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    match futures::join!(
        services::pages::get(&pool, "agence"),
        services::employees::get_all(&pool)
    ) {
        (Ok(page), Ok(employees)) => {
            crate::controllers::metrics::add(&req, &pool, page.id).await;

            #[derive(Template)]
            #[template(path = "agency.html")]
            struct Agency {
                title: String,
                description: Option<String>,
                employees: Vec<Employee>,
            }

            let page = Agency {
                title: page.title,
                description: page.description,
                employees: employees
                    .iter()
                    .map(|employee| Employee::from((*employee).clone()))
                    .collect::<Vec<Employee>>(),
            };

            match page.render() {
                Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/portfolio")]
async fn portfolio(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "portfolio").await {
        crate::controllers::metrics::add(&req, &pool, page.id).await;

        #[derive(Template)]
        #[template(path = "portfolio.html")]
        struct Portfolio {
            title: String,
            description: Option<String>,
        }

        let page = Portfolio {
            title: page.title,
            description: page.description,
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/mentions-legales")]
async fn legals(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "mentions_legales").await {
        crate::controllers::metrics::add(&req, &pool, page.id).await;

        #[derive(Template)]
        #[template(path = "legals.html")]
        struct Legals {
            title: String,
        }

        let page = Legals { title: page.title };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/sitemap.xml")]
pub async fn sitemap() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/xml")
        .body(include_str!("../../sitemap.xml"))
}

#[get("/robots.txt")]
pub async fn robots() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(include_str!("../../robots.txt"))
}
