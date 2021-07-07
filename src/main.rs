use actix_files::NamedFile;
use actix_web::{middleware::Compress, web, App, HttpRequest, HttpServer, HttpResponse, Result, http::{HeaderValue, header::{EXPIRES, CACHE_CONTROL}}};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::path::PathBuf;

mod controllers;
mod services;
mod templates;
mod utils;

async fn files(req: &HttpRequest, folder: &str) -> Result<HttpResponse, actix_web::Error> {
    use chrono::{Local, Duration};

    let path: PathBuf = format!("{}{}", folder, req.path())
        .parse()
        .unwrap();
    let file = NamedFile::open(path)?;
    let mut response = file.use_last_modified(true).into_response(&req).unwrap();
    let now = Local::now() + Duration::days(30);
    let headers = response.headers_mut();
    headers.append(EXPIRES, HeaderValue::from_str(&now.to_rfc2822()).unwrap());
    headers.append(CACHE_CONTROL, HeaderValue::from_static("public"));

    Ok(response)
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

    let pool: sqlx::postgres::PgPool = sqlx::pool::PoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not found"))
        .await
        .expect("Connection to database failed");

    // TODO : set ssl key path in .env
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("SSL build");
    builder
        .set_private_key_file(&std::env::var("PRIVATE_KEY_FILE").expect("PRIVATE_KEY_FILE not found in variables environment"), SslFiletype::PEM)
        .expect("private key file not found");
    builder
        .set_certificate_chain_file(&std::env::var("CERTIFICATE_CHAIN_FILE").expect("CERTIFICATE_CHAIN_FILE not found in variables environment"))
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
            .service(controllers::index)
            .service(controllers::agency)
            .service(controllers::creation_site_web)
            .service(controllers::portfolio)
            .service(
                web::scope("/contact")
                    .service(controllers::contact::page)
                    .service(controllers::contact::send),
            )
            .service(controllers::legals)
            .service(controllers::sitemap)
            .service(controllers::robots)
            .route(
                "/static/{filename:.*}",
                web::get().to(|req: HttpRequest| async move {
                    files(&req, ".").await
                }),
            )
            .route(
                "/uploads/{filename:.*}",
                web::get().to(|req: HttpRequest| async move {
                    files(&req, ".").await
                }),
            )
            .route(
                "/{filename:.*}",
                web::get().to(|req: HttpRequest| async move {
                    const FOLDER: &str = if cfg!(debug_assertions) {
                        ".build/development/"
                    } else {
                        "."
                    };
                    files(&req, FOLDER).await
                })
            )
            // .service(files)
    })
    .bind_openssl(
        // &std::env::var("SERVER_ADDR").expect("Cannot found server_addr"),
        &format!("{}:{}", server_addr, HTTPS_PORT),
        builder,
    )
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
