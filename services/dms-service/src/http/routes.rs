use actix_web::{web, Scope};
use crate::security::permission::RequirePermission;

pub fn api_scope()->Scope{
  web::scope("")
    .service(crate::http::handlers::health::healthz)
    .service(
      web::resource("/api/v1/dms/objects:presign-upload")
        .wrap(RequirePermission::new(crate::security::policy::perm::DMS_OBJECT_UPLOAD))
        .route(web::post().to(crate::http::handlers::upload::presign_upload)))
    .service(
      web::resource("/api/v1/dms/objects:presign-download")
        .wrap(RequirePermission::new(crate::security::policy::perm::DMS_OBJECT_DOWNLOAD))
        .route(web::post().to(crate::http::handlers::download::presign_download)))
    .service(
      web::resource("/api/v1/dms/signatures:attach")
        .wrap(RequirePermission::new(crate::security::policy::perm::DMS_SIGNATURE_ATTACH))
        .route(web::post().to(crate::http::handlers::sign::attach)))
}
