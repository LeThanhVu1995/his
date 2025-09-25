pub mod handlers;
pub mod routes;

use actix_web::web;

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::api_scope());
}
