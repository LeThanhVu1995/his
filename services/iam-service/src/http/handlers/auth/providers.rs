use actix_web::{web, HttpResponse};
use crate::config::types::ServiceConfig;

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct ProvidersResp { pub providers: Vec<String> }

#[utoipa::path(get, path = "/api/iam/auth/providers", tag = "iam", responses((status=200, body = ProvidersResp)))]
pub async fn providers(cfg: web::Data<ServiceConfig>) -> HttpResponse {
    HttpResponse::Ok().json(ProvidersResp { providers: cfg.idp_providers.clone() })
}


