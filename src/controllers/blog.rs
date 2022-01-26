use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::models;

#[get("")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) =
        models::pages::get::<super::Page>(&pool, "id, title, description", "/blog").await
    {
        if let (Ok(categories), Ok(posts), Ok(metric_id)) = futures::join!(
            models::blog::categories::get_all(&pool),
            models::blog::posts::get_latest(&pool, None),
            crate::controllers::metrics::add(
                &req,
                &pool,
                models::metrics::BelongsTo::Page(page.id)
            )
        ) {
            let mut token: Option<String> = None;

            if let Some(id) = metric_id {
                token = Some(id.to_string());
            }

            #[derive(Template)]
            #[template(path = "components/blog_post.html")]
            struct Post {
                name: String,
                uri: String,
                date: String,
                international_date: String,
                cover_filename: String,
                cover_path: String,
            }

            #[derive(Template)]
            #[template(path = "pages/blog/index.html")]
            struct Index {
                title: String,
                description: Option<String>,
                categories: Vec<models::blog::categories::Category>,
                posts: Vec<Post>,
                metrics_token: Option<String>,
            }

            let page = Index {
                title: page.title,
                description: page.description,
                categories,
                posts: posts
                    .iter()
                    .map(|post| Post {
                        name: post.name.clone(),
                        uri: post.uri.clone(),
                        date: post.date.format("%d/%m/%Y").to_string(),
                        international_date: post.date.to_rfc3339(),
                        cover_filename: {
                            let cover = post.cover.split('.').collect::<Vec<_>>();
                            cover.get(0).expect("Cannot get filename").to_string()
                        },
                        cover_path: post.cover.clone(),
                    })
                    .collect::<Vec<_>>(),
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                return HttpResponse::Ok().content_type("text/html").body(content);
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/articles/{name}-{id}")]
pub async fn show_article(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    web::Path((name, id)): web::Path<(String, i16)>,
) -> HttpResponse {
    if !models::blog::posts::exists(&pool, id).await
        || !models::blog::posts::is_published(&pool, id).await
    {
        return HttpResponse::NotFound().finish();
    }

    if let Ok((post, categories)) = futures::try_join!(
        models::blog::posts::get(&pool, id),
        models::blog::categories::get_all(&pool)
    ) {
        #[derive(Template)]
        #[template(path = "pages/blog/post.html")]
        struct Post {
            name: String,
            description: Option<String>,
            categories: Vec<models::blog::categories::Category>,
            content: String,
            cover_filename: String,
            cover_path: String,
            // image_path: String,
            date: String,
            international_date: String,
            author: models::blog::posts::Author,
        }

        let page = Post {
            name: post.name,
            description: post.description,
            categories,
            content: post.content,
            cover_filename: {
                let cover = post.cover.split('.').collect::<Vec<_>>();
                cover.get(0).expect("Cannot get filename").to_string()
            },
            cover_path: post.cover,
            // image_path: "/img/showcase.jpg".to_owned(),
            date: post.date.format("%d/%m/%Y").to_string(),
            international_date: post.date.to_rfc3339(),
            author: models::blog::posts::Author {
                fullname: post.author.fullname,
                picture: {
                    if let Some(picture) = post.author.picture {
                        let picture = picture.split('.').collect::<Vec<_>>();

                        if let Some(filename) = picture.get(0) {
                            Some(filename.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
            },
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/categories/{name}-{id}")]
pub async fn show_category(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    web::Path((name, id)): web::Path<(String, i16)>,
) -> HttpResponse {
    if !models::blog::categories::exists(&pool, id).await {
        return HttpResponse::NotFound().finish();
    }

    // let mut transaction = pool.begin().await.unwrap();
    // pool: impl sqlx::Executor<'_, Database = sqlx::Postgres>

    if let Ok((current_category, categories, posts)) = futures::try_join!(
        models::blog::categories::get(&pool, id),
        models::blog::categories::get_all(&pool),
        models::blog::posts::get_latest(&pool, Some(id))
    ) {
        #[derive(Template)]
        #[template(path = "components/blog_post.html")]
        struct Post {
            name: String,
            uri: String,
            date: String,
            international_date: String,
            cover_filename: String,
            cover_path: String,
        }

        #[derive(Template)]
        #[template(path = "pages/blog/category.html")]
        struct Index {
            title: String,
            description: Option<String>,
            categories: Vec<models::blog::categories::Category>,
            posts: Vec<Post>,
            metrics_token: Option<String>,
        }

        let page = Index {
            title: current_category.name,
            description: current_category.description,
            categories,
            posts: posts
                .iter()
                .map(|post| Post {
                    name: post.name.clone(),
                    uri: post.uri.clone(),
                    date: post.date.format("%d/%m/%Y").to_string(),
                    international_date: post.date.to_rfc3339(),
                    cover_filename: {
                        let cover = post.cover.split('.').collect::<Vec<_>>();
                        cover.get(0).expect("Cannot get filename").to_string()
                    },
                    cover_path: post.cover.clone(),
                })
                .collect::<Vec<_>>(),
            metrics_token: None,
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    // if let Ok(category) = models::blog::categories::get(&pool, id).await {
    //     let
    // }

    HttpResponse::InternalServerError().finish()
}
