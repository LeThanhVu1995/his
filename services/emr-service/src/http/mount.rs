use actix_web::web;

pub fn mount(cfg: &mut web::ServiceConfig) {
    crate::http::routes::api_scope(cfg);
}
