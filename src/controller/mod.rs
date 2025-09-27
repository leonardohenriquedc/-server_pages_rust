pub mod blogs;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(blogs::all_post).service(blogs::get_post);
}
