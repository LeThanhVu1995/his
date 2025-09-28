use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Notification {
    pub notification_id: Uuid,
    pub code: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct NotificationTarget {
    pub notification_id: Uuid,
    pub user_id: Uuid,
    pub read_at: Option<DateTime<Utc>>,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateNotificationRequest {
    pub code: Option<String>,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateNotificationRequest {
    pub code: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateNotificationTargetRequest {
    pub notification_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct MarkAsReadRequest {
    pub notification_id: Uuid,
    pub user_id: Uuid,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct NotificationQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
    pub user_id: Option<Uuid>,
    pub unread_only: Option<bool>,
}

// Response DTOs with additional info
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct NotificationWithTarget {
    pub notification_id: Uuid,
    pub code: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub is_read: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct NotificationStats {
    pub total: i64,
    pub unread: i64,
    pub read: i64,
}
