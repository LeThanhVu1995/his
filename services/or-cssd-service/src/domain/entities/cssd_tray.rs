use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CssdTray {
    pub tray_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CssdTrayItem {
    pub tray_item_id: Uuid,
    pub tray_id: Uuid,
    pub instrument_code: String,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CssdSterilizationLot {
    pub lot_id: Uuid,
    pub lot_code: String,
    pub method_code: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub released_by: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CssdLotItem {
    pub lot_item_id: Uuid,
    pub lot_id: Uuid,
    pub tray_id: Uuid,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateCssdTrayRequest {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateCssdTrayRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateCssdTrayItemRequest {
    pub tray_id: Uuid,
    #[validate(length(min = 1, message = "Instrument code cannot be empty"))]
    pub instrument_code: String,
    #[validate(range(min = 1, message = "Quantity must be greater than 0"))]
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateCssdTrayItemRequest {
    pub instrument_code: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateCssdSterilizationLotRequest {
    #[validate(length(min = 1, message = "Lot code cannot be empty"))]
    pub lot_code: String,
    #[validate(length(min = 1, message = "Method code cannot be empty"))]
    pub method_code: String,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateCssdSterilizationLotRequest {
    pub completed_at: Option<DateTime<Utc>>,
    pub released_by: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateCssdLotItemRequest {
    pub lot_id: Uuid,
    pub tray_id: Uuid,
    pub expiry_date: Option<NaiveDate>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CssdTrayQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CssdTrayItemQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub tray_id: Option<Uuid>,
    pub instrument_code: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CssdSterilizationLotQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub lot_code: Option<String>,
    pub method_code: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CssdLotItemQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub lot_id: Option<Uuid>,
    pub tray_id: Option<Uuid>,
    pub expiry_date_from: Option<NaiveDate>,
    pub expiry_date_to: Option<NaiveDate>,
}

// Response DTOs with additional info
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CssdTrayWithItems {
    pub tray_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub items: Vec<CssdTrayItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CssdSterilizationLotWithItems {
    pub lot_id: Uuid,
    pub lot_code: String,
    pub method_code: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub released_by: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub items: Vec<CssdLotItem>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CssdStats {
    pub total_trays: i64,
    pub total_lots: i64,
    pub active_lots: i64,
    pub completed_lots: i64,
    pub expired_items: i64,
}
