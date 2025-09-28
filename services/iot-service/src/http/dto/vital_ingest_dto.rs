use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct IngestVitalRequest {
    pub device_code: String,
    pub payload: Value,
}
