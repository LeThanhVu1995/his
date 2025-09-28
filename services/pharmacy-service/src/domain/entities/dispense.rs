use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Dispense {
    pub dispense_id: Uuid,
    pub prescription_id: Uuid,
    pub dispensed_by: Option<Uuid>,
    pub dispensed_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DispenseItem {
    pub dispense_item_id: Uuid,
    pub dispense_id: Uuid,
    pub prescription_item_id: Uuid,
    pub quantity: f64,
    pub unit: Option<String>,
    pub batch_id: Option<Uuid>,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

// Request DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateDispenseRequest {
    pub prescription_id: Uuid,
    pub dispensed_by: Option<Uuid>,
    pub items: Vec<CreateDispenseItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateDispenseItemRequest {
    pub prescription_item_id: Uuid,
    #[validate(range(min = 0.001, message = "Quantity must be greater than 0"))]
    pub quantity: f64,
    pub unit: Option<String>,
    pub batch_id: Option<Uuid>,
    pub expiry_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateDispenseRequest {
    pub status: Option<String>,
    pub dispensed_by: Option<Uuid>,
    pub dispensed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateDispenseItemRequest {
    #[validate(range(min = 0.001, message = "Quantity must be greater than 0"))]
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub batch_id: Option<Uuid>,
    pub expiry_date: Option<NaiveDate>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate, serde::Serialize, sqlx::FromRow)]
pub struct DispenseQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub prescription_id: Option<Uuid>,
    pub dispensed_by: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct DispenseItemQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub dispense_id: Option<Uuid>,
    pub prescription_item_id: Option<Uuid>,
    pub batch_id: Option<Uuid>,
    pub expiry_date_from: Option<NaiveDate>,
    pub expiry_date_to: Option<NaiveDate>,
}

// Response DTOs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DispenseWithItems {
    #[serde(flatten)]
    pub dispense: Dispense,
    pub items: Vec<DispenseItem>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DispenseStats {
    pub total: i64,
    pub in_progress: i64,
    pub completed: i64,
    pub cancelled: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ExpiredDispenseItem {
    pub dispense_item_id: Uuid,
    pub dispense_id: Uuid,
    pub prescription_item_id: Uuid,
    pub drug_name: String,
    pub batch_id: Uuid,
    pub expiry_date: NaiveDate,
    pub quantity: f64,
    pub unit: String,
}
