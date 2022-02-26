use crate::models;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use askama::Template;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use serde::Deserialize;
use sqlx::PgPool;
use std::fmt;
use std::ops::DerefMut;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ContactService {
    Website,
    VisualIdentity,
    MotionDesign,
}
impl fmt::Display for ContactService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Website => write!(f, "Site-web"),
            Self::VisualIdentity => write!(f, "Identité visuelle"),
            Self::MotionDesign => write!(f, "Motion design"),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoundBy {
    Ads,
    Friend,
    WebSearch,
    SocialNetwork,
    Website,
}

impl fmt::Display for FoundBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ads => write!(f, "Publicité"),
            Self::WebSearch => write!(f, "Recherche sur le web"),
            Self::Friend => write!(f, "Proche/Amie.e/Collègue"),
            Self::SocialNetwork => write!(f, "Réseau social"),
            Self::Website => write!(f, "Site-web"),
        }
    }
}

#[derive(Deserialize)]
pub struct Email {
    new_project: bool,
    firstname: String,
    lastname: String,
    company: Option<String>,
    email: String,
    phone: Option<String>,
    services: Option<Vec<ContactService>>,
    message: String,
    budget: Option<f64>,
    found_by: Option<FoundBy>,
}
impl Email {
    fn trim_strings(&mut self) {
        self.firstname = self.firstname.trim().to_owned();
        self.lastname = self.lastname.trim().to_owned();
        if let Some(company) = &mut self.company {
            self.company = Some(company.trim().to_owned())
        }
        self.email = self.email.trim().to_owned();
        if let Some(phone) = &mut self.phone {
            self.phone = Some(phone.trim().to_owned())
        }
        self.message = self.message.trim().to_owned();
    }
}

#[post("")]
pub async fn send(mut form: web::Json<Email>) -> Result<HttpResponse, Error> {
    form.trim_strings();

    let mut content = String::new();

    if form.firstname.len() < 2 || form.firstname.len() > 120 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    content.push_str(&format!("<u>Nom</u> : {}<br />", form.lastname));

    if form.lastname.len() < 2 || form.lastname.len() > 120 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    content.push_str(&format!("<u>Prénom</u> : {}<br />", form.firstname));

    if let Some(phone) = &form.phone {
        content.push_str(&format!("<u>N° de téléphone</u> : {}<br />", phone));
    }

    if form.new_project {
        if let Some(budget) = form.budget {
            if budget < 0.0 {
                return Ok(HttpResponse::BadRequest().json("Budget cannot be negative"));
            }

            content.push_str(&format!("<u>Budget</u> : {}€<br />", budget));
        }

        if let Some(company) = &form.company {
            let length = company.len();

            if !(3..=120).contains(&length) {
                return Ok(HttpResponse::BadRequest().finish());
            }

            content.push_str(&format!("<u>Société</u> : {}<br /><br />", company));
        } else {
            return Ok(HttpResponse::BadRequest().finish());
        }
    }

    // Prestations de service
    if let Some(services) = &form.services {
        content.push_str("<u>Intéressé par</u> : <ul>");

        for service in services {
            content.push_str(&format!("<li>{}</li>", service));
        }

        content.push_str("</ul>");
    }

    // On teste la longueur du message
    if form.message.len() < 30 || form.message.len() > 500 {
        return Ok(HttpResponse::BadRequest()
            .json("The message must contain between 30 and 500 characters"));
    }

    if let Some(found_by) = &form.found_by {
        content.push_str(&format!("<u>Trouvé par</u> : {}<br />", found_by));
    }

    content.push_str(&format!("<u>Message</u> :<br />{}", form.message));

    let email = EmailBuilder::new()
        .to("contact@greenassembly.fr")
        .from(form.email.as_str())
        .subject(&format!(
            "{} : {} {}",
            match form.new_project {
                true => "Nouveau projet",
                _ => "Demande renseignements",
            },
            form.lastname.to_uppercase(),
            form.firstname
        ))
        .html(content.as_str())
        .build();
    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

    match email {
        Ok(mail) => {
            let e = mailer.send(mail.into());

            if e.is_err() {
                return Ok(HttpResponse::InternalServerError().json("Unable to send email"));
            }
        }
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }

    Ok(HttpResponse::Ok().json("Email send successfully"))
}

#[get("")]
pub async fn page(req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Ok((page, mut transaction)) =
        futures::try_join!(models::pages::get(&pool, "/contact"), pool.begin())
    {
        if let Ok(token) = crate::controllers::metrics::add(
            &req,
            transaction.deref_mut(),
            models::metrics::BelongsTo::Page(page.id),
        )
        .await
        {
            #[derive(Template)]
            #[template(path = "pages/contact.html")]
            struct Contact {
                title: String,
                description: Option<String>,
                metrics_token: Option<String>,
            }

            let page = Contact {
                title: page.title,
                description: page.description,
                metrics_token: token,
            };

            if let Ok(content) = page.render() {
                if transaction.commit().await.is_ok() {
                    return HttpResponse::Ok().content_type("text/html").body(content);
                }
            }
        }
    }

    HttpResponse::InternalServerError().finish()
}
