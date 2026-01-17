use actix_web::web::Json;
use actix_web::{HttpResponse, Responder, web};

use crate::entity::users;
use sea_orm::EntityTrait;

use sea_orm::{ActiveModelTrait, Set, NotSet};

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

#[derive(serde::Deserialize)]
pub struct CreateUserDto {
    name: String,
    email: String,
}

pub async fn create_user(
    data: web::Data<AppState>,
    user_data: Json<CreateUserDto>,
) -> impl Responder {
    let new_user = users::ActiveModel {
        id: NotSet,
        name: Set(user_data.name.clone()),
        email: Set(user_data.email.clone()),
    };

    match new_user.insert(&data.db).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_users(data: web::Data<AppState>) -> impl Responder {
    match users::Entity::find().all(&data.db).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateUserDto {
    name: Option<String>,
    email: Option<String>,
}

pub async fn update_user(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    user_data: Json<UpdateUserDto>,
) -> impl Responder {
    let id = path.into_inner();

    // search user
    let user = match users::Entity::find_by_id(id).one(&data.db).await {
        Ok(Some(u)) => u,
        Ok(None) => return HttpResponse::NotFound().body("User not found"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut active_user: users::ActiveModel = user.into();

    if let Some(name) = &user_data.name {
        active_user.name = Set(name.clone());
    }
    if let Some(email) = &user_data.email {
        active_user.email = Set(email.clone());
    }

    match active_user.update(&data.db).await {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
