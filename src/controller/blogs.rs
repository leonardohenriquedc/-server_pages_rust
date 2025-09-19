use actix_web::{HttpResponse, Responder, get, post, web};

use crate::service::get_blogs::{get_blog_post, get_links_of_all_blogs};

#[get("/")]
pub async fn all_post() -> impl Responder {
    let hello = get_links_of_all_blogs().await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(hello)
}

#[get("/post/{name}")]
pub async fn get_post(name: web::Path<(String,)>) -> impl Responder {
    let post = get_blog_post(name.into_inner().0).await;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(post)
}
