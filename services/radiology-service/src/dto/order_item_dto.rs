use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderItemRequest {
    pub rad_order_id: Uuid,
    pub proc_id: Uuid,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateOrderItemRequest {
    pub status: Option<String>,
    pub performed_at: Option<DateTime<Utc>>,
    pub performer_staff_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderItemQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub rad_order_id: Option<Uuid>,
    pub proc_id: Option<Uuid>,
    pub status: Option<String>,
    pub performer_staff_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OrderItemResponse {
    pub rad_order_item_id: Uuid,
    pub rad_order_id: Uuid,
    pub proc_id: Uuid,
    pub status: String,
    pub performed_at: Option<DateTime<Utc>>,
    pub performer_staff_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}
