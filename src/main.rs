use actix_files::NamedFile;
use actix_web::{
    get,
    http::{
        header::{CACHE_CONTROL, EXPIRES},
        HeaderValue,
    },
    middleware::Compress,
    web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::path::Path;

mod controllers;
mod middlewares;
mod services;
mod templates;
mod tests;
mod utils;

async fn create_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let pool: sqlx::PgPool = sqlx::pool::PoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not found"))
        .await?;

    Ok(pool)
}

fn serve_file(req: &HttpRequest, path: &Path, cache_duration: i64) -> Result<HttpResponse, Error> {
    match NamedFile::open(path) {
        Ok(file) => {
            use chrono::{Duration, Local};

            let mut response = file.into_response(&req)?;
            let now = Local::now() + Duration::days(cache_duration);
            let headers = response.headers_mut();
            headers.append(EXPIRES, HeaderValue::from_str(&now.to_rfc2822()).unwrap());
            headers.append(CACHE_CONTROL, HeaderValue::from_static("public"));

            Ok(response)
        }
        Err(_) => {
            use askama::Template;

            #[derive(Template)]
            #[template(path = "404.html")]
            struct NotFound;

            Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body(NotFound.render().unwrap()))
        }
    }
}

pub async fn ban_route(req: HttpRequest, pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    if let Some(ip) = req.connection_info().realip_remote_addr() {
        if !services::ips_banned::is_banned(&pool, ip).await {
            let (_, counter) = futures::join!(
                services::ips_banned::add(&pool, ip),
                services::ips_banned::count(&pool, ip)
            );

            println!("Ban x{} times", counter);

            return HttpResponse::Ok()
                .content_type("text/plain; charset=utf-8")
                .body("Détection d'intrustion, par sécurité l'accès au site-web vous est bloqué pour votre IP pendant 5 minutes.");
        }
    }

    HttpResponse::NotFound().finish()
}

#[get("/{filename:.*}")]
async fn serve_public_file(req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut file_path = format!("./public/{}", req.path());
    let path = if cfg!(debug_assertions) {
        let mut path = Path::new(&file_path);

        if !path.exists() {
            file_path = format!("./.build/development{}", req.path());
            path = Path::new(&file_path);
        }

        path
    } else {
        file_path = format!(".{}", req.path());
        Path::new(&file_path)
    };

    serve_file(&req, &path, 30)
}

#[get("/uploads/{filename:.*}")]
async fn serve_upload_file(req: HttpRequest) -> Result<HttpResponse, Error> {
    let file_path = format!(".{}", req.path());
    let path = Path::new(&file_path);
    serve_file(&req, &path, 30)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use dotenv::dotenv;

    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,sqlx=debug");
    env_logger::init();

    const HTTP_PORT: u32 = if cfg!(debug_assertions) { 8080 } else { 80 };
    const HTTPS_PORT: u32 = if cfg!(debug_assertions) { 8443 } else { 443 };
    let server_addr =
        std::env::var("SERVER_ADDR").expect("SERVER_ADDR variable not specified in .env file");

    let pool = create_pool().await.expect("Connection to database failed");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("SSL build");
    builder
        .set_private_key_file(
            &std::env::var("PRIVATE_KEY_FILE")
                .expect("PRIVATE_KEY_FILE not found in variables environment"),
            SslFiletype::PEM,
        )
        .expect("private key file not found");
    builder
        .set_certificate_chain_file(
            &std::env::var("CERTIFICATE_CHAIN_FILE")
                .expect("CERTIFICATE_CHAIN_FILE not found in variables environment"),
        )
        .expect("certificate chain file not found");

    // Redirect HTTP to HTTPS
    let http = HttpServer::new(move || {
        App::new().wrap(utils::https::RedirectHTTPS::with_replacements(&[(
            "8080".to_owned(),
            "8443".to_owned(),
        )]))
    })
    .bind(&format!("{}:{}", server_addr, HTTP_PORT))?
    .run();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Compress::default())
            .wrap(middlewares::ban::Ban { pool: pool.clone() })
            .service(controllers::index)
            .service(controllers::agency)
            .service(
                web::scope("/creation-site-web")
                    .service(controllers::website::website_creation)
                    .service(controllers::website::onepage)
                    .service(controllers::website::showcase)
                    .service(controllers::website::e_commerce),
            )
            .service(controllers::portfolio)
            .service(
                web::scope("/contact")
                    .service(controllers::contact::page)
                    .service(controllers::contact::send),
            )
            .route("/admin", web::get().to(ban_route))
            .route("/backup", web::get().to(ban_route))
            .service(controllers::legals)
            .service(controllers::sitemap)
            .service(controllers::robots)
            .service(serve_upload_file)
            .service(serve_public_file)
    })
    .bind_openssl(&format!("{}:{}", server_addr, HTTPS_PORT), builder)
    .expect("Cannot bind openssl")
    .run();

    println!(
        "Server running at https://{}:{}",
        &std::env::var("SERVER_ADDR").unwrap(),
        HTTPS_PORT
    );

    futures::future::try_join(http, server)
        .await
        .expect("Error await");

    Ok(())
}
