use actix_web::{web, HttpResponse, http::header::LOCATION};
use serde::Deserialize;
use url::Url;
use crate::config::types::ServiceConfig;

#[derive(Debug, Deserialize)]
pub struct AuthzQuery {
    pub provider: String,
    pub redirect_uri: String,
    #[serde(default)] pub state: Option<String>,
    #[serde(default)] pub scope: Option<String>,
    #[serde(default)] pub code_challenge: Option<String>,
    #[serde(default)] pub code_challenge_method: Option<String>,
    #[serde(default)] pub prompt: Option<String>,
}

#[utoipa::path(get, path = "/api/iam/auth/authorize", tag = "iam",
    params(
        ("provider" = String, Query),
        ("redirect_uri" = String, Query),
        ("state" = Option<String>, Query),
        ("scope" = Option<String>, Query),
        ("code_challenge" = Option<String>, Query),
        ("code_challenge_method" = Option<String>, Query),
        ("prompt" = Option<String>, Query)
    ),
    responses((status=302))
)]
pub async fn authorize(cfg: web::Data<ServiceConfig>, q: web::Query<AuthzQuery>) -> HttpResponse {
    let provider = q.provider.to_lowercase();
    if !cfg.idp_providers.iter().any(|p| p == &provider) {
        return HttpResponse::BadRequest().json(serde_json::json!({"error":"unsupported_provider"}));
    }

    let issuer = cfg.security.issuer.trim_end_matches('/');
    let mut url = Url::parse(&format!("{issuer}/protocol/openid-connect/auth")).expect("issuer url");

    let scope = q.scope.clone().unwrap_or_else(|| "openid profile email".to_string());

    url.query_pairs_mut()
        .append_pair("client_id", &cfg.oidc_login_client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", &q.redirect_uri)
        .append_pair("scope", &scope)
        .append_pair("kc_idp_hint", &provider);

    if let Some(state) = &q.state { url.query_pairs_mut().append_pair("state", state); }
    if let Some(cc) = &q.code_challenge { url.query_pairs_mut().append_pair("code_challenge", cc); }
    if let Some(ccm) = &q.code_challenge_method { url.query_pairs_mut().append_pair("code_challenge_method", ccm); }
    if let Some(prompt) = &q.prompt { url.query_pairs_mut().append_pair("prompt", prompt); }

    HttpResponse::Found().append_header((LOCATION, url.as_str())).finish()
}


