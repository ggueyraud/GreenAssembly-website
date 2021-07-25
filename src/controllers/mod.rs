use crate::templates::Employee;
use actix_web::{get, HttpResponse};
use askama::Template;

pub mod contact;
pub mod employees;
pub mod users;
pub mod website;

#[get("/")]
pub async fn index() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "index.html")]
    struct Index;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(Index.render().unwrap())
}

#[get("/agence")]
async fn agency() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "agency.html")]
    struct Agency {
        employees: Vec<Employee>,
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(Agency {
            employees: vec![
                Employee {
                    fullname: String::from("Guillaume Gueyraud"),
                    position_held: String::from("Dirigeant • Développeur"),
                    description: String::from("Parce que la fiabilité d’un site-web fait toute la différence, je mets un point d’honneur à créer des sites web sécurisés et performants, répondant en tout point à vos attentes."),
                    picture: String::from("/uploads/gg")
                },
                Employee {
                    fullname: String::from("Vincent Bréhaut"),
                    position_held: String::from("Dirigeant • Développeur"),
                    description: String::from("C’est un fait, les internautes accordent peu de temps à leurs recherches sur le web. D’où l’importance d’avoir un site-web réactif, clair et intuitif, tout en réduisant au maximum l’impact écologique."),
                    picture: String::from("/uploads/vb")
                },
                Employee {
                    fullname: String::from("Ludivine Farat"),
                    position_held: String::from("Designer graphique"),
                    description: String::from("J’ai à cœur de trouver ce petit plus qui fait que vous êtes vous et pas un autre, et je le retranscris dans l’ensemble de votre communication visuelle."),
                    picture: String::from("/uploads/lf")
                },
            ]
        }.render().unwrap())
}

// #[get("/creation-site-web")]
// async fn creation_site_web() -> HttpResponse {
//     #[derive(Template)]
//     #[template(path = "website_creation.html")]
//     struct CreationSiteWeb;

//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(CreationSiteWeb.render().unwrap())
// }

#[get("/portfolio")]
async fn portfolio() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "portfolio.html")]
    struct Porfolio;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(Porfolio.render().unwrap())
}

#[get("/mentions-legales")]
async fn legals() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "legals.html")]
    struct Agency;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(Agency.render().unwrap())
}

#[get("/sitemap.xml")]
pub async fn sitemap() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/xml")
        .body(include_str!("../../sitemap.xml"))
}

#[get("/robots.txt")]
pub async fn robots() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(include_str!("../../robots.txt"))
}
