use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::services;

#[get("")]
pub async fn index(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok(page) = services::pages::get::<super::Page>(&pool, "id, title, description", "blog").await {
        // TODO : refactor this behavior
        crate::controllers::metrics::add(&req, &pool, services::metrics::BelongsTo::Page(page.id)).await;

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
#[get("/articles/{name}-{id}")]
pub async fn get_article(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String, i32)>,
) -> HttpResponse {
    let (article_name, article_id) = path.into_inner();

    // data get by the bdd
    use serde::Serialize;
    #[derive(Serialize)]
    struct ArticleData {
        title: String,
        description: Option<String>,
        image_path: String,
        // etc...
    }
    let article_data = ArticleData {
        title: "Titre de l'article".to_owned(),
        description: Some("Description de l'article".to_owned()),
        image_path: "/img/showcase.jpg".to_owned(),
    };

    #[derive(Template)]
    #[template(path = "blog/view_article.html")]
    struct Index {
        title: String,
        description: Option<String>,
        image_path: String,
    }

    let page = Index {
        title: article_data.title,
        description: article_data.description,
        image_path: article_data.image_path,
    };

    if let Ok(content) = page.render() {
        return HttpResponse::Ok().content_type("text/html").body(content);
    }

    HttpResponse::InternalServerError().finish()
}
