use actix_identity::Identity;
use actix_web::{get, HttpResponse};
use askama::Template;

#[get("")]
pub async fn dashboard(session: Identity) -> HttpResponse {
    if session.identity().is_some() {
        #[derive(Template)]
        #[template(path = "pages/admin/dashboard.html")]
        struct Dashboard;

        if let Ok(content) = Dashboard.render() {
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