use crate::auth::signup::signup;
use actix_web::{web, App, HttpServer}; // Import the signup function
mod auth;
mod db;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/signup", web::post().to(signup)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
