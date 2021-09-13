use crate::services::employees;
use crate::utils::error_logger::ErrorLogger;
use actix_web::{delete, patch, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
pub struct AddEmployee {
    pub firstname: String,
    pub lastname: String,
    pub job: String,
    pub description: Option<String>,
    pub drawing_order: Option<i16>,
}

#[post("")]
pub async fn add(pool: web::Data<PgPool>, data: web::Json<AddEmployee>) -> HttpResponse {
    match employees::insert(
        &pool,
        &data.firstname,
        &data.lastname,
        &data.job,
        data.description.as_deref(),
        data.drawing_order,
    )
    .await
    {
        Ok(id) => {
            // TODO //
            /*
                Penser à faire l'upload de l'image
            */
            HttpResponse::Ok().json(format!("{{id: {}}}", id))
        }
        Err(e) => {
            ErrorLogger::write(
                "employees_service",
                format!("Error {}: ", line!()) + &e.to_string(),
            );

            HttpResponse::InternalServerError().json(
                "Une erreur est survenue lors de la
                        création de l'employé",
            )
        }
    }
}

#[derive(Deserialize)]
pub struct EditEmployee {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub job: Option<String>,
    pub description: Option<String>,
    pub drawing_order: Option<i16>,
}

#[patch("/{id}")]
pub async fn edit_employee(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    form: web::Json<EditEmployee>,
) -> HttpResponse {
    if employees::exist(&pool, *id).await {
        match employees::update(
            &pool,
            *id,
            form.firstname.as_deref(),
            form.lastname.as_deref(),
            form.job.as_deref(),
            form.description.as_deref(),
            form.drawing_order,
        )
        .await
        {
            Ok(_) => {
                // TODO //
                /*
                    Penser à faire la mise à jour de l'image
                */

                return HttpResponse::Ok().json("Employé modifié avec succès");
            }
            Err(e) => {
                ErrorLogger::write(
                    "employees_service",
                    format!("Error {}: ", line!()) + &e.to_string(),
                );

                return HttpResponse::InternalServerError().json(
                    "Une erreur est survenue lors de la
                        modification de l'employé",
                );
            }
        }
    }

    HttpResponse::NotFound().finish()
}

#[delete("/{id}")]
pub async fn delete(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    if employees::exist(&pool, *id).await {
        return HttpResponse::Ok().json(employees::delete(&pool, *id).await);

        // TODO //
        /*
            Penser à faire la suppression de l'image
        */
    }

    HttpResponse::NotFound().finish()
}
