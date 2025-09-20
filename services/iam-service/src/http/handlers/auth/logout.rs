use actix_web::{web, HttpResponse, http::header::LOCATION};
use serde::Deserialize;
use url::Url;
use crate::config::types::ServiceConfig;

#[derive(Debug, Deserialize)]
pub struct LogoutQuery {
    pub redirect_uri: String,
    #[serde(default)] pub id_token_hint: Option<String>,
}

#[utoipa::path(get, path = "/api/iam/auth/logout", tag = "iam",
    params(("redirect_uri"=String, Query,), ("id_token_hint"=Option<String>, Query,)),
    responses((status=302))
)]
pub async fn logout(cfg: web::Data<ServiceConfig>, q: web::Query<LogoutQuery>) -> HttpResponse {
    let issuer = cfg.security.issuer.trim_end_matches('/');
    let mut url = Url::parse(&format!("{issuer}/protocol/openid-connect/logout")).expect("issuer url");
    url.query_pairs_mut().append_pair("post_logout_redirect_uri", &q.redirect_uri);
    if let Some(idh) = &q.id_token_hint { url.query_pairs_mut().append_pair("id_token_hint", idh); }
    HttpResponse::Found().append_header((LOCATION, url.as_str())).finish()
}


