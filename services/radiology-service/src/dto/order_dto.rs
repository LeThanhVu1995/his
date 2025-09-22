use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub procedure_id: Uuid,
    pub reason: Option<String>,
    pub priority: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateOrderReq {
    pub status: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OrderQuery {
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderRes {
    pub id: Uuid,
    pub order_no: String,
    pub status: String,
}
