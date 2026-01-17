use actix_web::web;
use crate::handlers::users::{get_user_by_id, create_user, get_all_users, update_user, delete_user};

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_all_users))
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user_by_id))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user))
    );
}
