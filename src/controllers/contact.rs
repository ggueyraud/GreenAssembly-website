use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use serde::Deserialize;
use std::fmt;

use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

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
pub struct Mail {
    new_project: bool,
    firstname: String,
    lastname: String,
    company: Option<String>,
    email: String,
    phone: Option<String>,
    services: Option<Vec<ContactService>>,
    message: String,
    budget: Option<f64>,
}
impl Mail {
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
pub async fn send(mut mail: web::Json<Mail>) -> Result<HttpResponse, Error> {
    mail.trim_strings();

    let mut content = String::new();

    if mail.firstname.len() < 2 || mail.firstname.len() > 120 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    content.push_str(&format!("<u>Nom</u> : {}<br />", mail.lastname));

    if mail.lastname.len() < 2 || mail.lastname.len() > 120 {
        return Ok(HttpResponse::BadRequest().finish());
    }

    content.push_str(&format!("<u>Prénom</u> : {}<br />", mail.firstname));

    if let Some(phone) = &mail.phone {
        content.push_str(&format!("<u>N° de téléphone</u> : {}<br />", phone));
    }

    if mail.new_project {
        if let Some(budget) = mail.budget {
            if budget < 0.0 {
                return Ok(HttpResponse::BadRequest().json("Budget cannot be negative"));
            }

            content.push_str(&format!("<u>Budget</u> : {}€<br />", budget));
        }

        if let Some(company) = &mail.company {
            let length = company.len();

            if length < 3 || length > 120 {
                return Ok(HttpResponse::BadRequest().finish());
            }

            content.push_str(&format!("<u>Société</u> : {}<br /><br />", company));
        } else {
            return Ok(HttpResponse::BadRequest().finish());
        }
    }

    // Prestations de service
    if let Some(services) = &mail.services {
        content.push_str("<u>Intéressé par</u> : <ul>");

        for service in services {
            content.push_str(&format!("<li>{}</li>", service));
        }

        content.push_str("</ul>");
    }

    // On teste la longueur du message
    if mail.message.len() < 30 || mail.message.len() > 500 {
        return Ok(HttpResponse::BadRequest()
            .json("The message must contain between 30 and 500 characters"));
    }

    content.push_str(&format!("<u>Message</u> :<br />{}", mail.message));

    let email = EmailBuilder::new()
        .to("contact@greenassembly.fr")
        .from(mail.email.as_str())
        .subject(&format!(
            "{} : {} {}",
            match mail.new_project {
                true => "Nouveau projet",
                _ => "Demande renseignements",
            },
            mail.lastname.to_uppercase(),
            mail.firstname
        ))
        .html(content.as_str())
        .build();
    let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();

    match email {
        Ok(mail) => {
            if mailer.send(mail.into()).is_err() {
                return Ok(HttpResponse::InternalServerError().json("Unable to send email"));
            }
        }
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }

    Ok(HttpResponse::Ok().json("Email send successfully"))
}

#[get("")]
pub async fn page() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "contact.html")]
    struct Contact;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(Contact.render().unwrap())
}
