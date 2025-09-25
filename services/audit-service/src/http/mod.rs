pub mod routes;
pub mod handlers;
pub mod dto;

pub fn mount(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(crate::http::routes::api_scope());
}
