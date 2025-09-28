use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Prescription {
    pub prescription_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub prescriber_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PrescriptionItem {
    pub prescription_item_id: Uuid,
    pub prescription_id: Uuid,
    pub drug_id: Uuid,
    pub dose_per_take: Option<f64>,
    pub dose_unit: Option<String>,
    pub frequency_text: Option<String>,
    pub route_code: Option<String>,
    pub duration_days: Option<i32>,
    pub quantity: Option<f64>,
    pub quantity_unit: Option<String>,
    pub instructions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

// Request DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePrescriptionRequest {
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub prescriber_id: Option<Uuid>,
    pub status: Option<String>,
    pub items: Vec<CreatePrescriptionItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePrescriptionItemRequest {
    pub drug_id: Uuid,
    pub dose_per_take: Option<f64>,
    pub dose_unit: Option<String>,
    pub frequency_text: Option<String>,
    pub route_code: Option<String>,
    pub duration_days: Option<i32>,
    pub quantity: Option<f64>,
    pub quantity_unit: Option<String>,
    pub instructions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePrescriptionRequest {
    pub status: Option<String>,
    pub prescriber_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePrescriptionItemRequest {
    pub dose_per_take: Option<f64>,
    pub dose_unit: Option<String>,
    pub frequency_text: Option<String>,
    pub route_code: Option<String>,
    pub duration_days: Option<i32>,
    pub quantity: Option<f64>,
    pub quantity_unit: Option<String>,
    pub instructions: Option<String>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate, serde::Serialize, sqlx::FromRow)]
pub struct PrescriptionQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub prescriber_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct PrescriptionItemQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub prescription_id: Option<Uuid>,
    pub drug_id: Option<Uuid>,
}

// Response DTOs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PrescriptionWithItems {
    #[serde(flatten)]
    pub prescription: Prescription,
    pub items: Vec<PrescriptionItem>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PrescriptionStats {
    pub total: i64,
    pub active: i64,
    pub completed: i64,
    pub cancelled: i64,
}
