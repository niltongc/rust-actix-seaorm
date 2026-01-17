use std::env;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

use crate::handlers::users::get_all_users;
use crate::handlers::users::get_user_by_id;
use crate::handlers::users::create_user;
use crate::handlers::users::update_user;

mod handlers;
mod entity;

struct AppState {
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");
    
    let db = Database::connect(database_url)
        .await
        .expect("DB connection failed");

    let app_state = web::Data::new(AppState { db });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users", web::post().to(create_user))
            .route("/users", web::get().to(get_all_users))
            .route("/users/{id}", web::put().to(update_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

