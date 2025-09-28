use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInsPayerRequest {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateInsPayerRequest {
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InsPayerResponse {
    pub payer_id: String,
    pub code: String,
    pub name: String,
}

impl InsPayerResponse {
    pub fn from_entity(payer: &crate::domain::entities::ins_payer::InsPayer) -> Self {
        Self {
            payer_id: payer.payer_id.clone(),
            code: payer.code.clone(),
            name: payer.name.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsPayersRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsPayersResponse {
    pub payers: Vec<InsPayerResponse>,
    pub total: i64,
}
