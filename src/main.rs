use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controller;
mod service;
mod structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Executando em: {}", env::current_dir().unwrap().display());
    HttpServer::new(|| App::new().configure(controller::config))
        .bind(("0.0.0.0", 7878))?
        .run()
        .await
}
