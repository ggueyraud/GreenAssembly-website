use actix_identity::Identity;
use actix_web::{get, HttpResponse, web};
use askama::Template;
use sqlx::PgPool;
use crate::models;

#[get("")]
pub async fn dashboard(session: Identity, pool: web::Data<PgPool>) -> HttpResponse {
    if session.identity().is_some() {
        #[derive(Template)]
        #[template(path = "pages/admin/dashboard.html")]
        struct Dashboard {
            pages: Vec<models::pages::Page>
        }

        let page = Dashboard {
            pages: models::pages::get_all(&pool).await.unwrap()
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