use crate::models;
use crate::templates::Employee;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

pub mod api;
pub mod admin;
pub mod blog;
pub mod contact;
pub mod metrics;
pub mod newsletter;
pub mod website;
pub mod users;

#[derive(sqlx::FromRow)]
struct Page {
    id: i16,
    title: String,
    description: Option<String>,
}

#[get("/")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get::<Page>(&pool, "id, title, description", "/").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "pages/index.html")]
        struct Index {
            title: String,
            description: Option<String>,
            metrics_token: Option<String>,
        }

        let page = Index {
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

#[get("/agence-digitale-verte")]
async fn agency(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let (Ok(page), employees) = futures::join!(
        models::pages::get::<Page>(&pool, "id, title, description", "/agence-digitale-verte"),
        models::users::get_employees(&pool)
    ) {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "pages/agency.html")]
        struct Agency {
            title: String,
            description: Option<String>,
            employees: Vec<Employee>,
            metrics_token: Option<String>,
        }

        let page = Agency {
            title: page.title,
            description: page.description,
            employees: employees
                .iter()
                .map(|employee| Employee::from((*employee).clone()))
                .collect::<Vec<Employee>>(),
            metrics_token: token,
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/portfolio")]
async fn portfolio(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) =
        models::pages::get::<Page>(&pool, "id, title, description", "/portfolio").await
    {
        #[derive(sqlx::FromRow)]
        struct Category {
            id: i16,
            name: String,
        }

        #[derive(sqlx::FromRow)]
        struct Project {
            id: i16,
            name: String,
            category_id: i16,
        }

        if let (Ok(metric_id), Ok(categories), Ok(projects)) = futures::join!(
            crate::controllers::metrics::add(
                &req,
                &pool,
                models::metrics::BelongsTo::Page(page.id)
            ),
            models::portfolio::categories::get_all::<Category>(&pool, "id, name"),
            models::portfolio::projects::get_all::<Project>(&pool, "id, name, category_id")
        ) {
            let mut token: Option<String> = None;

            if let Some(id) = metric_id {
                token = Some(id.to_string());
            }

            #[derive(Template)]
            #[template(path = "components/project.html")]
            struct Project {
                name: String,
                uri: String,
                illustration: String,
                fallback_illustration: String,
                category_id: i16,
            }

            #[derive(Template)]
            #[template(path = "pages/portfolio/index.html")]
            struct Portfolio {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
                categories: Vec<Category>,
                projects: Vec<Project>,
            }

            let mut cover_fut = vec![];

            for project in &projects {
                cover_fut.push(models::portfolio::pictures::get_cover(&pool, project.id));
            }

            if let Ok(covers) = futures::future::try_join_all(cover_fut).await {
                let mut page = Portfolio {
                    title: page.title,
                    description: page.description,
                    metrics_token: token,
                    categories,
                    projects: vec![],
                };

                for (i, cover) in covers.iter().enumerate() {
                    if let Some(project) = projects.get(i) {
                        page.projects.push(Project {
                            name: project.name.clone(),
                            fallback_illustration: format!(
                                "{}.webp",
                                cover.clone().split('.').collect::<Vec<_>>().get(0).unwrap()
                            ),
                            illustration: cover.to_string(),
                            uri: slugmin::slugify(&format!("{}-{}", project.name, project.id)),
                            category_id: project.category_id,
                        });
                    }
                }

                if let Ok(content) = page.render() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/portfolio/{name}-{id}")]
async fn show_project(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    web::Path((_, id)): web::Path<(String, i16)>,
) -> HttpResponse {
    if !models::portfolio::projects::exists(&pool, id).await {
        return HttpResponse::NotFound().finish();
    }

    #[derive(sqlx::FromRow)]
    struct Picture {
        path: String,
    }

    #[derive(sqlx::FromRow)]
    struct Project {
        name: String,
        description: Option<String>,
        content: String,
        is_visible: Option<bool>,
        date: chrono::DateTime<chrono::Utc>,
        international_date: String,
        last_update_date: Option<chrono::DateTime<chrono::Utc>>,
    }

    // println!("{:?}", metrics::add(&req, &pool, models::metrics::BelongsTo::Project(id)).await);
    // println!("{:?}",models::portfolio::projects::get::<Project>(
    //     &pool,
    //     r#"name, description, content, is_visible, date, TO_CHAR(date, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS international_date, last_update_date"#,
    //     id
    // ).await);
    // println!("{:?}", models::portfolio::pictures::get_all::<Picture>(&pool, "path", id).await);
    if let (Ok(metric_id), Ok(project), Ok(mut pictures)) = futures::join!(
        metrics::add(&req, &pool, models::metrics::BelongsTo::Project(id)),
        models::portfolio::projects::get::<Project>(
            &pool,
            r#"name, description, content, is_visible, date, TO_CHAR(date, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS international_date, last_update_date"#,
            id
        ),
        models::portfolio::pictures::get_all::<Picture>(&pool, "path", id)
    ) {
        let mut token: Option<String> = None;
        if let Some(id) = metric_id {
            token = Some(id.to_string());
        }

        struct Picture {
            filename: String,
            path: String,
        }

        #[derive(Template)]
        #[template(path = "pages/portfolio/project.html")]
        struct PortfolioProject {
            title: String,
            description: Option<String>,
            content: String,
            date: chrono::DateTime<chrono::Utc>,
            international_date: String,
            first_picture: Picture,
            pictures: Vec<Picture>,
            metrics_token: Option<String>,
        }

        let cover = pictures.remove(0).path;

        let project = PortfolioProject {
            title: project.name,
            description: project.description,
            content: project.content,
            date: project.date,
            international_date: project.international_date,
            first_picture: Picture {
                path: cover.clone(),
                filename: cover
                    .split('.')
                    .collect::<Vec<_>>()
                    .get(0)
                    .unwrap()
                    .to_string(),
            },
            pictures: pictures
                .iter()
                .map(|picture| Picture {
                    path: picture.path.clone(),
                    filename: picture
                        .path
                        .clone()
                        .split('.')
                        .collect::<Vec<_>>()
                        .get(0)
                        .unwrap()
                        .to_string(),
                })
                .collect::<Vec<_>>(),
            metrics_token: token,
        };

        if let Ok(content) = project.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/mentions-legales")]
async fn legals(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) =
        models::pages::get::<Page>(&pool, "id, title, description", "/mentions-legales").await
    {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

        #[derive(Template)]
        #[template(path = "pages/legals.html")]
        struct Legals {
            title: String,
            description: Option<String>,
            metrics_token: Option<String>,
        }

        let page = Legals {
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

#[get("/faq")]
async fn faq(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get::<Page>(&pool, "id, title, description", "/faq").await {
        let mut token: Option<String> = None;

        if let Ok(Some(id)) =
            crate::controllers::metrics::add(&req, &pool, models::metrics::BelongsTo::Page(page.id))
                .await
        {
            token = Some(id.to_string());
        }

        struct Category {
            id: i16,
            name: String,
            questions: Vec<models::faq::Answer>,
        }

        let mut categories = models::faq::categories::get_all(&pool)
            .await
            .iter_mut()
            .map(|category| Category {
                id: category.id,
                name: category.name.clone(),
                questions: vec![],
            })
            .collect::<Vec<_>>();

        for category in &mut categories {
            category.questions = models::faq::answers::get_all(&pool, category.id).await;
        }

        #[derive(Template)]
        #[template(path = "pages/faq.html")]
        struct FAQ {
            title: String,
            description: Option<String>,
            categories: Vec<Category>,
            metrics_token: Option<String>,
        }

        let page = FAQ {
            title: page.title,
            description: page.description,
            categories,
            metrics_token: token,
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
