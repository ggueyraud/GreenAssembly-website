use actix_web::{delete, get, patch, post, web, Error, HttpResponse};
use sqlx::postgres::PgPool;

use crate::services::employees;

#[post("/add")]
pub async fn add_employee(
    pool: web::Data<PgPool>,
    data: web::Json<employees::AddEmployee>,
) -> Result<HttpResponse, Error> {
    match employees::add_employee(&pool, &data).await {
        Ok(id) => Ok(HttpResponse::Ok().json(format!("{{id: {}}}", id))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    création de l'employé",
        )),
    }
    // TODO //
    /*
        Penser à faire l'upload de l'image
    */
}

#[patch("/edit")]
pub async fn edit_employee(
    pool: web::Data<PgPool>,
    data: web::Json<employees::EditEmployee>,
) -> Result<HttpResponse, Error> {
    match employees::is_employee_exist(&pool, data.id).await {
        Ok(true) => (),
        Ok(false) => return Ok(HttpResponse::NotFound().json("Employé non trouvé")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }

    match employees::edit_employee(&pool, &data).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Employé modifié avec succès")),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    modification de l'employé",
        )),
    }
    // TODO //
    /*
        Penser à faire la mise à jour de l'image
    */
}

#[delete("/delete")]
pub async fn delete_employee(
    pool: web::Data<PgPool>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let id = path.0 as i32;

    match employees::is_employee_exist(&pool, id).await {
        Ok(true) => (),
        Ok(false) => return Ok(HttpResponse::NotFound().json("Employé non trouvé")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
    match employees::delete_employee(&pool, id).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Employé supprimé avec succès")),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    suppression de l'employé",
        )),
    }
    // TODO //
    /*
        Penser à faire la suppression de l'image
    */
}

#[get("")]
pub async fn get_employees(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match employees::get_employees(&pool).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    récupération de la liste des employés",
        )),
    }
}
