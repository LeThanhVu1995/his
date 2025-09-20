// src/keycloak.rs placeholder
use std::collections::HashMap;
use std::time::Duration;

use app_error::AppError;
use app_web::prelude::{AuthTokenValidator, AuthUser};

use jsonwebtoken::{decode, decode_header, Algorithm, Validation};
use serde::Deserialize;

use crate::jwks::{JwksCache, JwksError};


#[derive(Clone, Debug)]
pub struct KeycloakValidatorConfig {
    pub issuer: String,            // ví dụ http://localhost:8080/realms/myapp
    pub audience: Option<String>,  // ví dụ "patient-api"
    pub jwks_ttl: Duration,        // ví dụ 10 phút
}

impl From<app_config::SecurityConfig> for KeycloakValidatorConfig {
    fn from(s: app_config::SecurityConfig) -> Self {
        let ttl = s
            .jwks_ttl
            .as_deref()
            .and_then(|x| humantime::parse_duration(x).ok())
            .unwrap_or_else(|| Duration::from_secs(600));
        Self {
            issuer: s.issuer,
            audience: if s.audience.is_empty() { None } else { Some(s.audience) },
            jwks_ttl: ttl,
        }
    }
}

#[derive(Clone)]
pub struct KeycloakValidator {
    cfg: KeycloakValidatorConfig,
    jwks: JwksCache,
}

impl KeycloakValidator {
    pub fn from_security_config(cfg: &app_config::SecurityConfig) -> Self {
        let kcfg: KeycloakValidatorConfig = cfg.clone().into();
        let endpoint = format!(
            "{}/protocol/openid-connect/certs",
            kcfg.issuer.trim_end_matches('/')
        );
        let jwks = JwksCache::new(endpoint, kcfg.jwks_ttl);
        Self { cfg: kcfg, jwks }
    }

    fn validation_template(&self) -> Validation {
        let mut v = Validation::new(Algorithm::RS256);
        v.set_issuer(&[self.cfg.issuer.clone()]);
        if let Some(aud) = &self.cfg.audience {
            v.set_audience(&[aud.clone()]);
        }
        v.leeway = 30; // 30s clock skew
        v
    }
}

/* ---------- Keycloak claims ---------- */
#[derive(Debug, Deserialize)]
struct RealmAccess {
    roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ResourceRoles {
    roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct KeycloakClaims {
    sub: String,
    #[serde(default)]
    preferred_username: Option<String>,

    #[serde(default)]
    scope: Option<String>,
    #[serde(default)]
    scp: Option<Vec<String>>,

    #[serde(default)]
    realm_access: Option<RealmAccess>,
    #[serde(default)]
    resource_access: Option<HashMap<String, ResourceRoles>>,

    #[serde(default)]
    iss: Option<String>,
    #[serde(default)]
    aud: Option<serde_json::Value>, // string or array, jsonwebtoken will validate audience
    #[serde(default)]
    azp: Option<String>,

    // exp, iat ... đã validate bởi jsonwebtoken
}

fn claims_to_auth_user(cl: &KeycloakClaims) -> AuthUser {
    let mut roles: Vec<String> = Vec::new();

    if let Some(r) = &cl.realm_access {
        roles.extend(r.roles.iter().cloned());
    }
    if let Some(map) = &cl.resource_access {
        for (_client, rr) in map.iter() {
            roles.extend(rr.roles.iter().cloned());
        }
    }

    // scopes
    let mut scopes: Vec<String> = Vec::new();
    if let Some(s) = &cl.scope {
        scopes.extend(s.split_whitespace().map(|x| x.to_string()));
    }
    if let Some(scp) = &cl.scp {
        scopes.extend(scp.iter().cloned());
    }

    AuthUser {
        user_id: cl.sub.clone(),
        subject: cl.sub.clone(),
        preferred_username: cl.preferred_username.clone(),
        roles,
        scopes,
        tenant_id: None,
    }
}

/* ---------- Implementations ---------- */

impl AuthTokenValidator for KeycloakValidator {
    async fn validate(&self, token: &str) -> Result<AuthUser, AppError> {
        // Parse header for kid/alg
        let header = decode_header(token).map_err(|_| AppError::Unauthorized)?;
        let kid = header.kid.ok_or(AppError::Unauthorized)?;
        if header.alg != Algorithm::RS256 {
            // Chỉ accept RS256 từ Keycloak
            return Err(AppError::Unauthorized);
        }

        // Lấy key từ cache/JWKS endpoint
        let key = match self.jwks.get_key_for_kid(&kid).await {
            Ok(k) => k,
            Err(JwksError::NoMatchingKey(_)) => {
                // thử refresh một lần nữa (race condition rotated keys)
                self.jwks.refresh().await.map_err(|_| AppError::Unauthorized)?;
                self.jwks.get_key_for_kid(&kid).await.map_err(|_| AppError::Unauthorized)?
            }
            Err(_) => return Err(AppError::Unauthorized),
        };

        // Validate token
        let validation = self.validation_template();
        let data = decode::<KeycloakClaims>(token, &key, &validation).map_err(|_| AppError::Unauthorized)?;

        Ok(claims_to_auth_user(&data.claims))
    }
}
