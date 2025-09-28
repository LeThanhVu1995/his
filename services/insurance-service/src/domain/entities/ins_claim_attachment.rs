use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsClaimAttachment {
    pub attach_id: String,
    pub claim_id: String,
    pub doc_id: String,
}
