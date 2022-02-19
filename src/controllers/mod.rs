use crate::templates::Employee;
use crate::{models, utils};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;
use std::ops::DerefMut;

pub mod admin;
pub mod api;
pub mod blog;
pub mod contact;
pub mod metrics;
pub mod newsletter;
pub mod users;
pub mod website;

#[get("/")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) =
        futures::try_join!(models::pages::get(&pool, "/"), pool.begin())
    {
        if let Ok(token) = metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
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
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/agence-digitale-verte")]
async fn agency(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) = futures::try_join!(
        models::pages::get(&pool, "/agence-digitale-verte"),
        pool.begin()
    ) {
        if let Ok((employees, token)) = futures::try_join!(
            models::users::get_employees(&pool),
            metrics::add(
                &req,
                transaction.deref_mut(),
                models::metrics::BelongsTo::Page(page.id)
            )
        ) {
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
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/portfolio")]
async fn portfolio(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) =
        futures::try_join!(models::pages::get(&pool, "/portfolio"), pool.begin())
    {
        if let Ok((token, _categories, projects)) = futures::try_join!(
            crate::controllers::metrics::add(
                &req,
                transaction.deref_mut(),
                models::metrics::BelongsTo::Page(page.id)
            ),
            models::portfolio::categories::get_all(&pool),
            models::portfolio::projects::get_all(&pool),
        ) {
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
                // categories: Vec<Category>,
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
                    // categories,
                    projects: vec![],
                };

                for (i, cover) in covers.iter().enumerate() {
                    if let Some(project) = projects.get(i) {
                        page.projects.push(Project {
                            name: project.name.clone(),
                            fallback_illustration: format!(
                                "{}.webp",
                                utils::extract_filename(&cover.clone())
                                    .expect("Cannot get filename"),
                            ),
                            illustration: cover.to_string(),
                            uri: slugmin::slugify(&format!("{}-{}", project.name, project.id)),
                            category_id: project.category_id,
                        });
                    }
                }

                if let Ok(content) = page.render() {
                    if transaction.commit().await.is_ok() {
                        return HttpResponse::Ok().content_type("text/html").body(content);
                    }
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
    if let Ok(mut transaction) = pool.begin().await {
        if !models::portfolio::projects::exists(&pool, id).await {
            return HttpResponse::NotFound().finish();
        }

        // If the project to be accessed isn't published, so we redirect to portfolio home page
        if !models::portfolio::projects::is_published(&pool, id).await {
            return HttpResponse::NotFound()
                .header("Location", "/portfolio")
                .finish();
        }

        if let Ok((token, project, mut pictures)) = futures::try_join!(
            metrics::add(
                &req,
                transaction.deref_mut(),
                models::metrics::BelongsTo::Project(id)
            ),
            models::portfolio::projects::get(&pool, id),
            models::portfolio::pictures::get_all(&pool, id)
        ) {
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
                first_picture: Picture,
                pictures: Vec<Picture>,
                metrics_token: Option<String>,
                is_seo: Option<bool>,
            }

            let cover = pictures.remove(0);

            let project = PortfolioProject {
                title: project.name,
                description: project.description,
                content: project.content,
                first_picture: Picture {
                    path: cover.clone(),
                    filename: crate::utils::extract_filename(&cover).unwrap(),
                },
                pictures: pictures
                    .iter()
                    .map(|picture| Picture {
                        path: picture.clone(),
                        filename: crate::utils::extract_filename(picture).unwrap(),
                    })
                    .collect::<Vec<_>>(),
                metrics_token: token,
                is_seo: project.is_seo,
            };

            if let Ok(content) = project.render() {
                return HttpResponse::Ok().content_type("text/html").body(content);
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/mentions-legales")]
async fn legals(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) =
        futures::try_join!(models::pages::get(&pool, "/mentions-legales"), pool.begin())
    {
        if let Ok(token) = metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
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
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/faq")]
async fn faq(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) =
        futures::try_join!(models::pages::get(&pool, "/faq"), pool.begin())
    {
        if let Ok((token, categories)) = futures::try_join!(
            metrics::add(
                &req,
                transaction.deref_mut(),
                models::metrics::BelongsTo::Page(page.id),
            ),
            models::faq::categories::get_all(&pool)
        ) {
            struct Category {
                id: i16,
                name: String,
                questions: Vec<models::faq::Answer>,
            }

            let mut categories = categories
                .iter()
                .map(|category| Category {
                    id: category.id,
                    name: category.name.clone(),
                    questions: vec![],
                })
                .collect::<Vec<_>>();

            for category in &mut categories {
                match models::faq::answers::get_all(&pool, category.id).await {
                    Ok(answers) => category.questions = answers,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                }
            }

            #[derive(Template)]
            #[template(path = "pages/faq.html")]
            struct Faq {
                title: String,
                description: Option<String>,
                categories: Vec<Category>,
                metrics_token: Option<String>,
            }

            let page = Faq {
                title: page.title,
                description: page.description,
                categories,
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
