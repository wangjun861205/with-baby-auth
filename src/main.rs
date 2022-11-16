pub mod core;
pub mod errors;
pub mod handlers;
pub mod hashers;
pub mod models;
pub mod schema;
pub mod storers;
pub mod tokeners;

#[macro_use]
extern crate diesel;

use actix_web::web::{get, post, Data};
use dotenv::dotenv;
use env_logger;
use hashers::SHA384Hasher;
use log::warn;
use storers::mongo::MongoStorer;
use tokeners::JWTTokener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    if let Err(e) = dotenv() {
        warn!("failed to load .env: {}", e);
    }
    let storer = MongoStorer::new(
        &dotenv::var("DATABASE_URL").expect("environment variable DATABASE_URL not exists"),
    )
    .await
    .expect("failed to create MongoStorer");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(
                JWTTokener::new(
                    &dotenv::var("JWT_KEY").expect("environment varialbe JWT_KEY not exists"),
                )
                .expect("failed to create JWTTokener"),
            ))
            .app_data(Data::new(SHA384Hasher {}))
            .app_data(Data::new(storer.clone()))
            .route("signup", post().to(handlers::signup))
            .route("signin", post().to(handlers::signin))
            .route("verify", get().to(handlers::verify))
            .route("exists", get().to(handlers::exists))
    })
    .bind(format!(
        "{}",
        dotenv::var("ADDRESS").unwrap_or("0.0.0.0:8000".into())
    ))
    .expect("failed to bind server address")
    .run()
    .await
}
