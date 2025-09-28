use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LabTestCatalog {
    pub test_id: Uuid,
    pub code: String,
    pub name: String,
    pub specimen_code: Option<String>,
    pub method_text: Option<String>,
    pub loinc_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LabOrder {
    pub lab_order_id: Uuid,
    pub order_id: Uuid,
    pub collected_at: Option<DateTime<Utc>>,
    pub collected_by: Option<Uuid>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LabOrderItem {
    pub lab_order_item_id: Uuid,
    pub lab_order_id: Uuid,
    pub test_id: Uuid,
    pub status: String,
    pub resulted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LabResult {
    pub lab_result_id: Uuid,
    pub lab_order_item_id: Uuid,
    pub result_status: String,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LabResultValue {
    pub value_id: Uuid,
    pub lab_result_id: Uuid,
    pub analyte_code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<Decimal>,
    pub ref_high: Option<Decimal>,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateLabTestRequest {
    pub code: String,
    pub name: String,
    pub specimen_code: Option<String>,
    pub method_text: Option<String>,
    pub loinc_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateLabTestRequest {
    pub name: Option<String>,
    pub specimen_code: Option<String>,
    pub method_text: Option<String>,
    pub loinc_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateLabOrderRequest {
    pub order_id: Uuid,
    pub collected_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateLabOrderRequest {
    pub collected_at: Option<DateTime<Utc>>,
    pub collected_by: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateLabOrderItemRequest {
    pub lab_order_id: Uuid,
    pub test_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateLabOrderItemRequest {
    pub status: Option<String>,
    pub resulted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateLabResultRequest {
    pub lab_order_item_id: Uuid,
    pub result_status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateLabResultRequest {
    pub result_status: Option<String>,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateLabResultValueRequest {
    pub lab_result_id: Uuid,
    pub analyte_code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<Decimal>,
    pub ref_high: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateLabResultValueRequest {
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<Decimal>,
    pub ref_high: Option<Decimal>,
}

// Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LabOrderResponse {
    pub lab_order_id: Uuid,
    pub order_id: Uuid,
    pub collected_at: Option<DateTime<Utc>>,
    pub collected_by: Option<Uuid>,
    pub status: String,
    pub items: Vec<LabOrderItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LabOrderItemResponse {
    pub lab_order_item_id: Uuid,
    pub lab_order_id: Uuid,
    pub test_id: Uuid,
    pub test: LabTestCatalog,
    pub status: String,
    pub resulted_at: Option<DateTime<Utc>>,
    pub result: Option<LabResultResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LabResultResponse {
    pub lab_result_id: Uuid,
    pub lab_order_item_id: Uuid,
    pub result_status: String,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub remarks: Option<String>,
    pub values: Vec<LabResultValue>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LabTestQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
    pub specimen_code: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LabOrderQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub order_id: Option<Uuid>,
    pub status: Option<String>,
    pub patient_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LabResultQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub lab_order_item_id: Option<Uuid>,
    pub result_status: Option<String>,
    pub patient_id: Option<Uuid>,
}
