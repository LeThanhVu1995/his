// routes placeholder
use actix_web::web;

use super::handlers;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get().to(handlers::health::health))
        .service(
            web::scope("/me")
                .route("", web::get().to(handlers::me::me))
        )
        .service(
            web::scope("/users")
                .route("", web::get().to(handlers::users::list))
                .route("", web::post().to(handlers::users::create))
                .route("/{id}", web::get().to(handlers::users::get))
                .route("/{id}", web::put().to(handlers::users::update))
                .route("/{id}/lock", web::post().to(handlers::users::lock))
        )
        .service(
            web::scope("/roles")
                .route("", web::get().to(handlers::roles::list))
                .route("/assign", web::post().to(handlers::roles::assign))
        );
}

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg
      .route("/api/iam/health", web::get().to(handlers::health::health))
      .service(
        web::scope("/api/iam/auth")
          .route("/providers", web::get().to(handlers::auth::providers::providers))
          .route("/authorize", web::get().to(handlers::auth::authorize::authorize))
          .route("/logout", web::get().to(handlers::auth::logout::logout))
      );
}
