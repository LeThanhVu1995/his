use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateResultRequest {
    pub rad_order_item_id: Uuid,
    pub report_text: Option<String>,
    pub result_status: Option<String>,
    pub pacs_study_uid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateResultRequest {
    pub report_text: Option<String>,
    pub result_status: Option<String>,
    pub reported_at: Option<DateTime<Utc>>,
    pub reported_by: Option<Uuid>,
    pub pacs_study_uid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResultQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub rad_order_item_id: Option<Uuid>,
    pub result_status: Option<String>,
    pub reported_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResultResponse {
    pub rad_result_id: Uuid,
    pub rad_order_item_id: Uuid,
    pub report_text: Option<String>,
    pub result_status: String,
    pub reported_at: Option<DateTime<Utc>>,
    pub reported_by: Option<Uuid>,
    pub pacs_study_uid: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}
