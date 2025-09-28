use sha2::{Digest, Sha256};
use serde::Deserialize;
use utoipa::ToSchema;

pub fn calc_etag(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("\"{:x}\"", h.finalize())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
