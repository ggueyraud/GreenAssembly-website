use crate::models;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use sqlx::PgPool;

#[get("")]
pub async fn dashboard(session: Identity, pool: web::Data<PgPool>) -> HttpResponse {
    if session.identity().is_some() {
        #[derive(Template)]
        #[template(path = "pages/admin/dashboard.html")]
        struct Dashboard {
            pages: Vec<models::pages::Page>,
        }

        let page = Dashboard {
            pages: models::pages::get_all(&pool).await.unwrap(),
        };

        if let Ok(content) = page.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    } else {
        #[derive(Template)]
        #[template(path = "pages/admin/login.html")]
        struct Login;

        if let Ok(content) = Login.render() {
            return HttpResponse::Ok().content_type("text/html").body(content);
        }
    }

    HttpResponse::InternalServerError().finish()
}

#[get("/blog")]
pub async fn blog(session: Identity, _pool: web::Data<PgPool>) -> HttpResponse {
    if session.identity().is_none() {
        return HttpResponse::Found().header("location", "/admin").finish();
    }

    // if let Ok((categories, posts)) = futures::try_join!(
    //     models::blog::categories::get_all(&pool),
    //     models::blog::posts::get_all(&pool, None)
    // ) {
    //     #[derive(Template)]
    //     #[template(path = "pages/admin/blog.html")]
    //     struct Blog {
    //         categories: Vec<models::blog::categories::CategoryAdminInformations>,
    //         posts: Vec<models::blog::posts::PostAdminInformations>
    //     }

    //     let page = Blog {
    //         categories,
    //         posts
    //     };

    //     if let Ok(content) =page.render() {
    //         return HttpResponse::Ok().content_type("text/html").body(content);
    //     }
    // }

    HttpResponse::InternalServerError().finish()
}
