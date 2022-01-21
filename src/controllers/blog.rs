use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::models;

#[get("")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) =
        models::pages::get::<super::Page>(&pool, "id, title, description", "/blog").await
    {
        #[derive(sqlx::FromRow, Debug)]
        struct Category {
            name: String,
            uri: String,
        }

        #[derive(sqlx::FromRow, Debug)]
        struct Post {
            name: String,
            description: Option<String>,
            uri: String,
            date: String,
            international_date: String,
        }

        if let (Ok(categories), Ok(posts), Ok(metric_id)) = futures::join!(
            models::blog::categories::get_all::<Category>(&pool, "name, uri"),
            models::blog::posts::get_all_published::<Post>(
                &pool,
                r#"bp.name,
                bp.description,
                uri,
                TO_CHAR(bp.date, 'DD/MM/YYYY') AS "date",
                TO_CHAR(bp.date, 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS international_date"#
            ),
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
                description: Option<String>,
                uri: String,
                date: String,
                international_date: String,
            }

            #[derive(Template)]
            #[template(path = "pages/blog/index.html")]
            struct Index {
                title: String,
                description: Option<String>,
                categories: Vec<Category>,
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
                        description: post.description.clone(),
                        uri: post.uri.clone(),
                        date: post.date.clone(),
                        international_date: post.international_date.clone(),
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

    if let Ok(post) = models::blog::posts::get(&pool, id).await {
        println!("{:?}", post);

        #[derive(Template)]
        #[template(path = "pages/blog/post.html")]
        struct Post {
            name: String,
            description: Option<String>,
            content: String,
            image_path: String,
            date: String,
            international_date: String,
            author: models::blog::posts::Author,
        }

        let page = Post {
            name: post.name,
            description: post.description,
            content: post.content,
            image_path: "/img/showcase.jpg".to_owned(),
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
