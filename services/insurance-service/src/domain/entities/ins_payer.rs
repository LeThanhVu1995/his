use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsPayer {
    pub payer_id: String,
    pub code: String,
    pub name: String,
}
