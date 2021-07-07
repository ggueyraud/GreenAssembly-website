use actix_web::{delete, get, patch, post, web, Error, HttpResponse};
use sqlx::postgres::PgPool;

use crate::services::users;
use crate::utils::encode_pwd::encode_pwd;

#[post("/add")]
pub async fn add_user(
    pool: web::Data<PgPool>,
    mut data: web::Json<users::AddUser>,
) -> Result<HttpResponse, Error> {
    data.password = encode_pwd(&data.password);

    match users::add_user(&pool, &data).await {
        Ok(id) => Ok(HttpResponse::Ok().json(format!("{{id: {}}}", id))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    création de l'utilisateur",
        )),
    }
}

#[patch("/edit")]
pub async fn edit_user(
    pool: web::Data<PgPool>,
    mut data: web::Json<users::EditUser>,
) -> Result<HttpResponse, Error> {
    match users::is_user_exist(&pool, data.id).await {
        Ok(true) => (),
        Ok(false) => return Ok(HttpResponse::NotFound().json("Utilisateur non trouvé")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
    // On crypte le nouveau mot de passe
    data.password = encode_pwd(&data.password);

    match users::edit_user(&pool, &data).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Utilisateur modifié avec succès")),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    modification de l'utilisateur",
        )),
    }
}

#[delete("/delete/{id}")]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<u32>,
) -> Result<HttpResponse, Error> {
    let id = path.0 as i32;

    match users::is_user_exist(&pool, id).await {
        Ok(true) => (),
        Ok(false) => return Ok(HttpResponse::NotFound().json("Utilisateur non trouvé")),
        Err(e) => return Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
    match users::delete_user(&pool, id).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Utilisateur supprimé avec succès")),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    suppression de l'utilisateur",
        )),
    }
}

#[get("")]
pub async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match users::get_users(&pool).await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(_) => Ok(HttpResponse::InternalServerError().json(
            "Une erreur est survenue lors de la
                    récupération de la liste des utilisateurs",
        )),
    }
}
