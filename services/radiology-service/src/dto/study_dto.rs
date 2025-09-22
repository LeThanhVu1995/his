use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateStudyReq {
    pub order_id: Uuid,
    pub modality: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProgressStudyReq {
    pub action: String, // START/END
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct StudyQuery {
    pub order_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StudyRes {
    pub id: Uuid,
    pub accession_no: String,
    pub status: String,
}
