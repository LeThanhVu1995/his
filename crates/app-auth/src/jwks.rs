// src/jwks.rs placeholder
use serde::Deserialize;
use std::time::{Duration, Instant};
use thiserror::Error;

use tokio::sync::RwLock;

use jsonwebtoken::DecodingKey;

#[derive(Debug, Error)]
pub enum JwksError {
    #[error("http error: {0}")]
    Http(String),
    #[error("jwks parse error: {0}")]
    Parse(String),
    #[error("no matching key for kid={0}")]
    NoMatchingKey(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    pub kid: String,
    pub kty: String, // "RSA"
    pub alg: Option<String>, // "RS256"
    pub n: Option<String>,   // modulus (base64url)
    pub e: Option<String>,   // exponent (base64url)
    // (bỏ qua x5c,...)
}

#[derive(Debug, Clone, Deserialize)]
pub struct Jwks {
    pub keys: Vec<Jwk>,
}

impl Jwk {
    pub fn to_decoding_key(&self) -> Option<DecodingKey> {
        match (&self.n, &self.e) {
            (Some(n), Some(e)) => {
                // jsonwebtoken::DecodingKey expects base64url n,e (JWK format)
                Some(DecodingKey::from_rsa_components(n, e).ok()?)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct JwksCache {
    endpoint: String,
    ttl: Duration,
    inner: std::sync::Arc<RwLock<Inner>>,
}

#[derive(Debug, Clone)]
struct Inner {
    jwks: Option<Jwks>,
    expires_at: Instant,
}

impl JwksCache {
    pub fn new(endpoint: impl Into<String>, ttl: Duration) -> Self {
        Self {
            endpoint: endpoint.into(),
            ttl,
            inner: std::sync::Arc::new(RwLock::new(Inner {
                jwks: None,
                expires_at: Instant::now(),
            })),
        }
    }

    fn expired(expires_at: Instant) -> bool {
        Instant::now() >= expires_at
    }

    pub async fn get_key_for_kid(&self, kid: &str) -> Result<DecodingKey, JwksError> {
        // 1) Try memory and not expired
        {
            let guard = self.inner.read().await;
            if let Some(jwks) = &guard.jwks {
                if !Self::expired(guard.expires_at) {
                    if let Some(j) = jwks.keys.iter().find(|k| k.kid == kid) {
                        if let Some(k) = j.to_decoding_key() {
                            return Ok(k);
                        }
                    }
                }
            }
        }

        // 2) Refresh
        self.refresh().await?;

        // 3) Try again
        let guard = self.inner.read().await;
        if let Some(jwks) = &guard.jwks {
            if let Some(j) = jwks.keys.iter().find(|k| k.kid == kid) {
                if let Some(k) = j.to_decoding_key() {
                    return Ok(k);
                }
            }
        }

        Err(JwksError::NoMatchingKey(kid.to_string()))
    }

    pub async fn refresh(&self) -> Result<(), JwksError> {
        let resp = reqwest::Client::new()
            .get(&self.endpoint)
            .send()
            .await
            .map_err(|e| JwksError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(JwksError::Http(format!(
                "status {} from {}",
                resp.status(),
                self.endpoint
            )));
        }
        let jwks: Jwks = resp
            .json()
            .await
            .map_err(|e| JwksError::Parse(e.to_string()))?;

        {
            let mut guard = self.inner.write().await;
            guard.jwks = Some(jwks);
            guard.expires_at = Instant::now() + self.ttl;
        }
        Ok(())
    }
}
