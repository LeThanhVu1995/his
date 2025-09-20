use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateOrderReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub order_type: String,
    pub priority: Option<String>,
    pub note: Option<String>,
    pub items: Vec<CreateItemReq>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateItemReq {
    pub item_code: String,
    pub item_name: String,
    pub quantity: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateOrderReq {
    pub priority: Option<String>,
    pub note: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateItemReq {
    pub item_name: Option<String>,
    pub quantity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct SubmitResultReq {
    pub result_json: serde_json::Value,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct OrderQuery {
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderRes {
    pub id: Uuid,
    pub order_no: String,
    pub order_type: String,
    pub status: String,
    pub priority: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItemRes {
    pub id: Uuid,
    pub item_code: String,
    pub item_name: String,
    pub quantity: i32,
    pub status: String,
}
