use std::env;

use actix_web::{App, HttpServer};

mod controller;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Executando em: {}", env::current_dir().unwrap().display());
    HttpServer::new(|| App::new().configure(controller::config))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}
