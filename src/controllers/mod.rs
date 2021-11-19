use crate::services;
use crate::templates::Employee;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

use sqlx::PgPool;
pub mod blog;
pub mod contact;
pub mod employees;
pub mod metrics;
pub mod users;
pub mod website;

#[get("/")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "/").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) = crate::controllers::metrics::add(&req, &pool, page.id).await {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "index.html")]
        struct Index {
            title: String,
            description: Option<String>,
            metrics_token: Option<String>
        }

        let page = Index {
            title: page.title,
            description: page.description,
            metrics_token: token
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/agence-digitale-verte")]
async fn agency(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    match futures::join!(
        services::pages::get(&pool, "/agence-digitale-verte"),
        services::employees::get_all(&pool)
    ) {
        (Ok(page), Ok(employees)) => {
            let mut token: Option<String> = None;

            if let Ok(Some(id)) = crate::controllers::metrics::add(&req, &pool, page.id).await {
                token = Some(id.to_string());
            }

            #[derive(Template)]
            #[template(path = "agency.html")]
            struct Agency {
                title: String,
                description: Option<String>,
                employees: Vec<Employee>,
                metrics_token: Option<String>
            }

            let page = Agency {
                title: page.title,
                description: page.description,
                employees: employees
                    .iter()
                    .map(|employee| Employee::from((*employee).clone()))
                    .collect::<Vec<Employee>>(),
                metrics_token: token
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
    if let Ok(page) = services::pages::get(&pool, "/portfolio").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) = crate::controllers::metrics::add(&req, &pool, page.id).await {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "portfolio.html")]
        struct Portfolio {
            title: String,
            description: Option<String>,
            metrics_token: Option<String>
        }

        let page = Portfolio {
            title: page.title,
            description: page.description,
            metrics_token: token
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/mentions-legales")]
async fn legals(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "/mentions-legales").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) = crate::controllers::metrics::add(&req, &pool, page.id).await {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "legals.html")]
        struct Legals {
            title: String,
            description: Option<String>,
            metrics_token: Option<String>
        }

        let page = Legals {
            title: page.title,
            description: page.description,
            metrics_token: token
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/faq")]
async fn faq(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get(&pool, "/faq").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) = crate::controllers::metrics::add(&req, &pool, page.id).await {
            token = Some(id.to_string());
        }

        struct Category {
            id: i16,
            name: String,
            questions: Vec<services::faq::Answer>,
        }

        let mut categories = services::faq::categories::get_all(&pool)
            .await
            .iter_mut()
            .map(|category| Category {
                id: category.id,
                name: category.name.clone(),
                questions: vec![],
            })
            .collect::<Vec<_>>();

        for category in &mut categories {
            category.questions = services::faq::answers::get_all(&pool, category.id).await;
        }

        #[derive(Template)]
        #[template(path = "faq.html")]
        struct FAQ {
            title: String,
            description: Option<String>,
            categories: Vec<Category>,
            metrics_token: Option<String>
        }

        let page = FAQ {
            title: page.title,
            description: page.description,
            categories,
            metrics_token: token
        };

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
