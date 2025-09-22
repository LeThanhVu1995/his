use actix_web::web;

pub mod routes;
pub mod handlers;

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(crate::http::routes::api_scope());
}
