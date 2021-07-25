use askama::Template;
use actix_web::{get, HttpResponse};

#[get("")]
pub async fn website_creation() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "website_creation.html")]
    struct CreationSiteWeb;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(CreationSiteWeb.render().unwrap())
}

#[get("/onepage")]
pub async fn onepage_website_creation() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "onepage_website.html")]
    struct CreationSiteWeb;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(CreationSiteWeb.render().unwrap())
}

#[get("/vitrine")]
pub async fn showcase_website_creation() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "showcase_website.html")]
    struct CreationSiteWeb;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(CreationSiteWeb.render().unwrap())
}

#[get("/e-commerce")]
pub async fn e_commerce_website_creation() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "e_commerce_website.html")]
    struct CreationSiteWeb;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(CreationSiteWeb.render().unwrap())
}