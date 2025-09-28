use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DigitalSignature {
    pub signature_id: String,
    pub signer_id: Option<String>,
    pub algorithm: Option<String>,
    pub signed_at: DateTime<Utc>,
    pub signature_b64: Option<String>,
}
