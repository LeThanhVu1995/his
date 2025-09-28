use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDateTime};
use validator::Validate;
use crate::http::dto::common::ApiResponse;

// Clinical Order DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[validate(length(min = 1, max = 36))]
    pub patient_id: String,

    #[validate(length(min = 1, max = 36))]
    pub encounter_id: String,

    #[validate(length(min = 1, max = 64))]
    pub order_type: String,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub ordered_by: Option<String>,

    pub ordered_at: DateTime<Utc>,

    pub priority_code: Option<String>,

    #[validate(length(max = 1000))]
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateOrderRequest {
    #[validate(length(max = 64))]
    pub order_type: Option<String>,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub ordered_by: Option<String>,

    pub ordered_at: Option<DateTime<Utc>>,

    pub priority_code: Option<String>,

    #[validate(length(max = 1000))]
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CompleteOrderRequest {
    pub completed_date: NaiveDateTime,

    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order_id: String,
    pub patient_id: String,
    pub encounter_id: String,
    pub order_type: String,
    pub order_code: String,
    pub order_name: String,
    pub status: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub order_date: Option<NaiveDateTime>,
    pub scheduled_date: Option<NaiveDateTime>,
    pub completed_date: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}

// Query DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ListOrderQuery {
    #[validate(length(max = 64))]
    pub order_type: Option<String>,

    #[validate(length(max = 64))]
    pub status: Option<String>,

    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// From entity implementations
impl OrderResponse {
    pub fn from_entity(entity: crate::domain::entities::order::ClinicalOrder) -> Self {
        Self::from(entity)
    }
}

impl From<crate::domain::entities::order::ClinicalOrder> for OrderResponse {
    fn from(entity: crate::domain::entities::order::ClinicalOrder) -> Self {
        Self {
            order_id: entity.order_id.clone(),
            patient_id: entity.patient_id,
            encounter_id: entity.encounter_id,
            order_type: entity.order_type.clone(),
            order_code: entity.order_id, // Use order_id as code
            order_name: entity.order_type, // Use order_type as name
            status: entity.status,
            description: None, // Not available in entity
            instructions: None, // Not available in entity
            order_date: Some(entity.ordered_at.naive_utc()),
            scheduled_date: None, // Not available in entity
            completed_date: None, // Not available in entity
            notes: entity.remarks,
            created_at: entity.ordered_at,
            updated_at: entity.ordered_at,
            created_by: entity.ordered_by,
        }
    }
}

// Type aliases for API responses
pub type OrderApiResponse = ApiResponse<OrderResponse>;
pub type OrderListResponse = ApiResponse<Vec<OrderResponse>>;
