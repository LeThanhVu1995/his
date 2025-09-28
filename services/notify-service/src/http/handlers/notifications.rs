use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::entities::notification::{
    Notification, NotificationTarget, NotificationWithTarget, NotificationStats,
    CreateNotificationRequest, UpdateNotificationRequest, CreateNotificationTargetRequest,
    MarkAsReadRequest, NotificationQuery
};
use crate::infrastructure::repositories::notification_repo::{NotificationRepo, NotificationTargetRepo};
use utoipa::ToSchema;

// Notification Management
#[utoipa::path(
    post,
    path = "/api/v1/notify/notifications",
    request_body = CreateNotificationRequest,
    responses(
        (status = 201, description = "Notification created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_notification(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse> {
    let notification_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let notification = Notification {
        notification_id,
        code: body.code.clone(),
        title: body.title.clone(),
        body: body.body.clone(),
        created_at: now,
    };

    NotificationRepo { db: &db }
        .create(&notification)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create notification"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": notification_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/notify/notifications",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("code" = Option<String>, Query, description = "Filter by notification code")
    ),
    responses(
        (status = 200, description = "List of notifications"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_notifications(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<NotificationQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let notifications = NotificationRepo { db: &db }
        .list_paged(query.code.clone(), page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(notifications))
}

#[utoipa::path(
    get,
    path = "/api/v1/notify/notifications/{id}",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification found"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_notification(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let notification_id = path.into_inner();

    let notification = NotificationRepo { db: &db }
        .get_by_id(notification_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Notification not found"))?;

    Ok(HttpResponse::Ok().json(notification))
}

#[utoipa::path(
    put,
    path = "/api/v1/notify/notifications/{id}",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    request_body = UpdateNotificationRequest,
    responses(
        (status = 200, description = "Notification updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_notification(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateNotificationRequest>,
) -> Result<HttpResponse> {
    let notification_id = path.into_inner();

    let mut notification = NotificationRepo { db: &db }
        .get_by_id(notification_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Notification not found"))?;

    // Apply updates
    if let Some(code) = body.code.clone() { notification.code = Some(code); }
    if let Some(title) = body.title.clone() { notification.title = title; }
    if let Some(body_text) = body.body.clone() { notification.body = Some(body_text); }

    NotificationRepo { db: &db }
        .update(notification_id, &notification)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update notification"))?;

    Ok(HttpResponse::Ok().json(notification))
}

#[utoipa::path(
    delete,
    path = "/api/v1/notify/notifications/{id}",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    responses(
        (status = 204, description = "Notification deleted successfully"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_notification(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let notification_id = path.into_inner();

    NotificationRepo { db: &db }
        .delete(notification_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to delete notification"))?;

    Ok(HttpResponse::NoContent().finish())
}

// Notification Targeting
#[utoipa::path(
    post,
    path = "/api/v1/notify/notifications/{id}/targets",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    request_body = CreateNotificationTargetRequest,
    responses(
        (status = 201, description = "Notification target created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn assign_notification(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<CreateNotificationTargetRequest>,
) -> Result<HttpResponse> {
    let notification_id = path.into_inner();

    let target = NotificationTarget {
        notification_id,
        user_id: body.user_id,
        read_at: None,
    };

    NotificationTargetRepo { db: &db }
        .create(&target)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to assign notification"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"success": true})))
}

#[utoipa::path(
    post,
    path = "/api/v1/notify/notifications/{id}/targets/bulk",
    params(
        ("id" = Uuid, Path, description = "Notification ID")
    ),
    request_body = Vec<Uuid>,
    responses(
        (status = 201, description = "Notifications assigned to users successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn assign_notification_bulk(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<Vec<Uuid>>,
) -> Result<HttpResponse> {
    let notification_id = path.into_inner();

    NotificationTargetRepo { db: &db }
        .assign_to_users(notification_id, body.into_inner())
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to assign notifications"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"success": true})))
}

// User Notifications
#[utoipa::path(
    get,
    path = "/api/v1/notify/users/{user_id}/notifications",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("unread_only" = Option<bool>, Query, description = "Show only unread notifications")
    ),
    responses(
        (status = 200, description = "User notifications"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_notifications(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    query: web::Query<NotificationQuery>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let notifications = NotificationTargetRepo { db: &db }
        .get_user_notifications(user_id, query.unread_only, page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(notifications))
}

#[utoipa::path(
    get,
    path = "/api/v1/notify/users/{user_id}/notifications/stats",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User notification statistics"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_user_notification_stats(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    let stats = NotificationTargetRepo { db: &db }
        .get_user_stats(user_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(stats))
}

#[utoipa::path(
    put,
    path = "/api/v1/notify/users/{user_id}/notifications/{notification_id}/read",
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
        ("notification_id" = Uuid, Path, description = "Notification ID")
    ),
    responses(
        (status = 200, description = "Notification marked as read"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn mark_notification_read(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse> {
    let (user_id, notification_id) = path.into_inner();

    NotificationTargetRepo { db: &db }
        .mark_as_read(notification_id, user_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to mark as read"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"success": true})))
}
