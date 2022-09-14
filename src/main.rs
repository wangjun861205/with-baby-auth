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
use hashers::SHA384Hasher;
use storers::PgStorer;
use tokeners::JWTTokener;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().expect("failed to load .env");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(Data::new(
                JWTTokener::new(&dotenv::var("JWT_KEY").expect("JWT_KEY not exists in .env"))
                    .expect("failed to create JWTTokener"),
            ))
            .app_data(Data::new(SHA384Hasher {}))
            .app_data(Data::new(
                PgStorer::new(
                    &dotenv::var("DATABASE_URL").expect("DATABASE_URL not exists in .env"),
                )
                .expect("failed to create PgStorer"),
            ))
            .route("signup", post().to(handlers::signup))
            .route("signin", post().to(handlers::signin))
            .route("verify", get().to(handlers::verify))
            .route("exists", get().to(handlers::exists))
    })
    .bind("0.0.0.0:8000")
    .expect("failed to bind server address")
    .run()
    .await
}
