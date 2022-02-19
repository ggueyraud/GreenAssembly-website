use crate::{models, utils};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

#[get("")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = models::pages::get(&pool, "/blog").await {
        if let Ok((categories, posts, token)) = futures::try_join!(
            models::blog::categories::get_all_visible(&pool),
            models::blog::posts::get_latest(&pool, None),
            crate::controllers::metrics::add(
                &req,
                pool.as_ref(),
                models::metrics::BelongsTo::Page(page.id)
            )
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
                        cover_filename: utils::extract_filename(&post.cover)
                            .expect("Cannot get filename"),
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
pub async fn show_post(
    // TODO : get metric
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    // TODO : check if name has changed so make a redirection 500
    web::Path((name, id)): web::Path<(String, i16)>,
) -> HttpResponse {
    // If any redirection exist for the post so current path is an old one
    if models::blog::redirections::exists(&pool, models::blog::redirections::Type::Post, id, &name)
        .await
    {
        return match models::blog::posts::get_uri(&pool, id).await {
            Ok(uri) => HttpResponse::MovedPermanently()
                .header(actix_web::http::header::LOCATION, uri)
                .finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
    }

    // If the uri neither a redirection or isn't the current uri or the post isn't published so it doesn't exist
    // TODO : handle is_seo
    if !models::blog::posts::exists(&pool, id, &name).await
        || !models::blog::posts::is_published(&pool, id).await
    {
        return HttpResponse::NotFound().finish();
    }

    if let Ok((post, categories)) = futures::try_join!(
        models::blog::posts::get(&pool, id),
        models::blog::categories::get_all_visible(&pool)
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
            date: String,
            international_date: String,
            author: models::blog::posts::Author,
        }

        let page = Post {
            name: post.name,
            description: post.description,
            categories,
            content: post.content,
            cover_filename: utils::extract_filename(&post.cover).expect("Cannot get filename"),
            cover_path: post.cover,
            date: post.date.format("%d/%m/%Y").to_string(),
            international_date: post.date.to_rfc3339(),
            author: models::blog::posts::Author {
                fullname: post.author.fullname,
                picture: post
                    .author
                    .picture
                    .and_then(|picture| utils::extract_filename(&picture)),
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
    // TODO : get metric
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    // TODO : check if name has changed so make a redirection 500
    web::Path((name, id)): web::Path<(String, i16)>,
) -> HttpResponse {
    // If any redirection exist for the category so current path is an old one
    if models::blog::redirections::exists(
        &pool,
        models::blog::redirections::Type::Category,
        id,
        &name,
    )
    .await
    {
        return match models::blog::categories::get_uri(&pool, id).await {
            Ok(uri) => HttpResponse::MovedPermanently()
                .header(actix_web::http::header::LOCATION, uri)
                .finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        };
    }

    if !models::blog::categories::exists(&pool, id, &name).await {
        return HttpResponse::NotFound().finish();
    }

    // let mut transaction = pool.begin().await.unwrap();
    // pool: impl sqlx::Executor<'_, Database = sqlx::Postgres>

    if let Ok((current_category, categories, posts)) = futures::try_join!(
        models::blog::categories::get(&pool, id),
        models::blog::categories::get_all_visible(&pool),
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
                    cover_filename: utils::extract_filename(&post.cover)
                        .expect("Cannot get filename"),
                    cover_path: post.cover.clone(),
                })
                .collect::<Vec<_>>(),
            metrics_token: None,
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}
