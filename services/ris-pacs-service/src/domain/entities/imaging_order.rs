use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ImagingOrder {
    pub imaging_order_id: Uuid,
    pub order_id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub procedure_id: Uuid,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub scheduled_room_id: Option<Uuid>,
    pub status: String,
    pub priority: String,
    pub reason: Option<String>,
    pub requested_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateImagingOrderRequest {
    pub order_id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub procedure_id: Uuid,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub scheduled_room_id: Option<Uuid>,
    pub priority: Option<String>,
    pub reason: Option<String>,
    pub requested_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateImagingOrderRequest {
    pub scheduled_at: Option<DateTime<Utc>>,
    pub scheduled_room_id: Option<Uuid>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImagingOrderQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    pub procedure_id: Option<Uuid>,
    pub scheduled_from: Option<NaiveDate>,
    pub scheduled_to: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ImagingOrderResponse {
    pub imaging_order_id: Uuid,
    pub order_id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub procedure_id: Uuid,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub scheduled_room_id: Option<Uuid>,
    pub status: String,
    pub priority: String,
    pub reason: Option<String>,
    pub requested_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}