use std::env;

use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

mod handlers;
mod entity;

mod app_config;
use crate::app_config::app_config;

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
            .configure(app_config)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

