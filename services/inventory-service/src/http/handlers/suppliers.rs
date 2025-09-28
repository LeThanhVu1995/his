use actix_web::{web, HttpResponse};
use actix_web::web::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::SupplierRepo;
use crate::dto::supplier_dto::{CreateSupplierDto, UpdateSupplierDto, SupplierQuery, SupplierDto};

#[utoipa::path(
    get,
    path = "/api/v1/inv/suppliers",
    params(
        ("q" = Option<String>, Query, description = "Search query"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_suppliers(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<SupplierQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (suppliers, total) = SupplierRepo { db: &db }
        .list_paged(query.q.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<SupplierDto> = suppliers.into_iter().map(|s| SupplierDto {
        id: s.id,
        code: s.code,
        name: s.name,
        phone: s.phone,
        email: s.email,
        address_line1: s.address_line1,
        address_line2: s.address_line2,
        city: s.city,
        province: s.province,
        country: s.country,
        postal_code: s.postal_code,
        tax_id: s.tax_id,
        status: s.status,
        created_at: s.created_at,
        updated_at: s.updated_at,
        created_by: s.created_by,
        updated_by: s.updated_by,
        deleted_at: s.deleted_at,
        deleted_by: s.deleted_by,
    }).collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/inv/suppliers:create",
    request_body = CreateSupplierDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_supplier(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateSupplierDto>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let supplier = crate::domain::models::Supplier {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        phone: payload.phone.clone(),
        email: payload.email.clone(),
        address_line1: payload.address_line1.clone(),
        address_line2: payload.address_line2.clone(),
        city: payload.city.clone(),
        province: payload.province.clone(),
        country: payload.country.clone(),
        postal_code: payload.postal_code.clone(),
        tax_id: payload.tax_id.clone(),
        status: "ACTIVE".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        created_by: None,
        updated_by: None,
        deleted_at: None,
        deleted_by: None,
    };

    SupplierRepo { db: &db }.create(&supplier).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(SupplierDto {
        id,
        code: supplier.code,
        name: supplier.name,
        phone: supplier.phone,
        email: supplier.email,
        address_line1: supplier.address_line1,
        address_line2: supplier.address_line2,
        city: supplier.city,
        province: supplier.province,
        country: supplier.country,
        postal_code: supplier.postal_code,
        tax_id: supplier.tax_id,
        status: supplier.status,
        created_at: supplier.created_at,
        updated_at: supplier.updated_at,
        created_by: supplier.created_by,
        updated_by: supplier.updated_by,
        deleted_at: supplier.deleted_at,
        deleted_by: supplier.deleted_by,
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/inv/suppliers/{id}",
    request_body = UpdateSupplierDto,
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_supplier(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateSupplierDto>,
) -> actix_web::Result<HttpResponse> {
    let rec = SupplierRepo { db: &db }.update(
        path.into_inner(),
        payload.name.as_deref(),
        payload.phone.as_deref(),
        payload.email.as_deref(),
    ).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(SupplierDto {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        phone: rec.phone,
        email: rec.email,
        address_line1: rec.address_line1,
        address_line2: rec.address_line2,
        city: rec.city,
        province: rec.province,
        country: rec.country,
        postal_code: rec.postal_code,
        tax_id: rec.tax_id,
        status: rec.status,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
        created_by: rec.created_by,
        updated_by: rec.updated_by,
        deleted_at: rec.deleted_at,
        deleted_by: rec.deleted_by,
    }))
}
