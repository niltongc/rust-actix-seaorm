use actix_web::{HttpResponse, Responder, web};

use crate::entity::users;
use sea_orm::EntityTrait;

use crate::AppState;

pub async fn get_user_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();

    match users::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}