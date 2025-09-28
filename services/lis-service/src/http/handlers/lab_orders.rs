use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::entities::lab_order::{
    LabTestCatalog, LabOrder, LabOrderItem, LabResult, LabResultValue,
    CreateLabTestRequest, UpdateLabTestRequest, CreateLabOrderRequest, UpdateLabOrderRequest,
    CreateLabOrderItemRequest, UpdateLabOrderItemRequest, CreateLabResultRequest, UpdateLabResultRequest,
    CreateLabResultValueRequest, UpdateLabResultValueRequest,
    LabTestQuery, LabOrderQuery, LabResultQuery
};
use crate::infra::db::repositories::lab_order_repo::{LabTestRepo, LabOrderRepo};
use validator::Validate;

// Lab Test Management
#[utoipa::path(
    post,
    path = "/api/v1/lab/tests:create",
    request_body = CreateLabTestRequest,
    responses(
        (status = 201, description = "Lab test created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_lab_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateLabTestRequest>,
) -> Result<HttpResponse> {
    let test_id = Uuid::new_v4();
    let test = LabTestCatalog {
        test_id,
        code: body.code.clone(),
        name: body.name.clone(),
        specimen_code: body.specimen_code.clone(),
        method_text: body.method_text.clone(),
        loinc_code: body.loinc_code.clone(),
    };

    LabTestRepo { db: &db }
        .create(&test)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create lab test"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": test_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/lab/tests",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("code" = Option<String>, Query, description = "Filter by test code"),
        ("specimen_code" = Option<String>, Query, description = "Filter by specimen code")
    ),
    responses(
        (status = 200, description = "List of lab tests"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_lab_tests(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<LabTestQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let tests = LabTestRepo { db: &db }
        .list_paged(query.code.clone(), query.specimen_code.clone(), page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(tests))
}

#[utoipa::path(
    get,
    path = "/api/v1/lab/tests/{id}",
    params(
        ("id" = Uuid, Path, description = "Test ID")
    ),
    responses(
        (status = 200, description = "Lab test found"),
        (status = 404, description = "Lab test not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_lab_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let test_id = path.into_inner();

    let test = LabTestRepo { db: &db }
        .get_by_id(test_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Lab test not found"))?;

    Ok(HttpResponse::Ok().json(test))
}

#[utoipa::path(
    put,
    path = "/api/v1/lab/tests/{id}",
    params(
        ("id" = Uuid, Path, description = "Test ID")
    ),
    request_body = UpdateLabTestRequest,
    responses(
        (status = 200, description = "Lab test updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Lab test not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_lab_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateLabTestRequest>,
) -> Result<HttpResponse> {
    let test_id = path.into_inner();

    let mut test = LabTestRepo { db: &db }
        .get_by_id(test_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Lab test not found"))?;

    // Apply updates
    if let Some(name) = body.name.clone() { test.name = name; }
    if let Some(specimen_code) = body.specimen_code.clone() { test.specimen_code = Some(specimen_code); }
    if let Some(method_text) = body.method_text.clone() { test.method_text = Some(method_text); }
    if let Some(loinc_code) = body.loinc_code.clone() { test.loinc_code = Some(loinc_code); }

    LabTestRepo { db: &db }
        .update(test_id, &test)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update lab test"))?;

    Ok(HttpResponse::Ok().json(test))
}

// Lab Order Management
#[utoipa::path(
    post,
    path = "/api/v1/lab/orders:create",
    request_body = CreateLabOrderRequest,
    responses(
        (status = 201, description = "Lab order created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_lab_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateLabOrderRequest>,
) -> Result<HttpResponse> {
    let lab_order_id = Uuid::new_v4();
    let order = LabOrder {
        lab_order_id,
        order_id: body.order_id,
        collected_at: None,
        collected_by: body.collected_by,
        status: "PLACED".to_string(),
    };

    LabOrderRepo { db: &db }
        .create(&order)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create lab order"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": lab_order_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/lab/orders/{id}",
    params(
        ("id" = Uuid, Path, description = "Lab Order ID")
    ),
    responses(
        (status = 200, description = "Lab order found"),
        (status = 404, description = "Lab order not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_lab_order(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let lab_order_id = path.into_inner();

    let order = LabOrderRepo { db: &db }
        .get_by_id(lab_order_id)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("Lab order not found"))?;

    Ok(HttpResponse::Ok().json(order))
}

#[utoipa::path(
    get,
    path = "/api/v1/lab/orders",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("order_id" = Option<Uuid>, Query, description = "Filter by order ID"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of lab orders"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_lab_orders(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<LabOrderQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    let orders = LabOrderRepo { db: &db }
        .list_paged(query.order_id, query.status.clone(), page_size, offset)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;

    Ok(HttpResponse::Ok().json(orders))
}
