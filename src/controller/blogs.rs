use actix_web::{HttpResponse, Responder, get};

use crate::service::get_blogs::hello_word;

#[get("/")]
pub async fn hello() -> impl Responder {
    let hello = hello_word().await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(hello)
}
