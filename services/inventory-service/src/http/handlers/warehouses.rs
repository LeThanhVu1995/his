use actix_web::{web, HttpResponse};
use actix_web_validator::{Query, Json};
use uuid::Uuid;
use crate::domain::repo::WarehouseRepo;
use crate::dto::warehouse_dto::{CreateWarehouseReq, UpdateWarehouseReq, WarehouseQuery, WarehouseRes};
use crate::dto::common::calc_etag;

// #[utoipa::path(
//     get,
//     path = "/api/v1/inv/warehouses",
//     params(
//         ("q" = Option<String>, Query),
//         ("page" = Option<i64>, Query),
//         ("page_size" = Option<i64>, Query)
//     ),
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn list_warehouses(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: Query<WarehouseQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    let (warehouses, total) = WarehouseRepo { db: &db }
        .search_paged(query.q.as_deref(), page, page_size)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    let response: Vec<WarehouseRes> = warehouses.into_iter().map(|w| WarehouseRes {
        id: w.id,
        code: w.code,
        name: w.name,
        r#type: w.r#type,
    }).collect();

    let json = serde_json::to_vec(&response).map_err(|_| crate::error::AppError::Internal("JSON".into()))?;
    let etag = calc_etag(&json);

    Ok(HttpResponse::Ok()
        .append_header(("ETag", etag))
        .append_header(("X-Total-Count", total.to_string()))
        .json(response))
}

// #[utoipa::path(
//     post,
//     path = "/api/v1/inv/warehouses:create",
//     request_body = CreateWarehouseReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn create_warehouse(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateWarehouseReq>,
) -> actix_web::Result<HttpResponse> {
    let id = Uuid::new_v4();
    let w = crate::domain::models::Warehouse {
        id,
        code: payload.code.clone(),
        name: payload.name.clone(),
        r#type: payload.r#type.clone().unwrap_or_else(|| "MAIN".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    WarehouseRepo { db: &db }.create(&w).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(WarehouseRes {
        id,
        code: w.code,
        name: w.name,
        r#type: w.r#type,
    }))
}

// #[utoipa::path(
//     put,
//     path = "/api/v1/inv/warehouses/{id}",
//     request_body = UpdateWarehouseReq,
//     security(
//         ("bearer_auth" = [])
//     )
// )]
pub async fn update_warehouse(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateWarehouseReq>,
) -> actix_web::Result<HttpResponse> {
    let rec = WarehouseRepo { db: &db }.update(
        path.into_inner(),
        payload.name.as_deref(),
        payload.r#type.as_deref(),
    ).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(WarehouseRes {
        id: rec.id,
        code: rec.code,
        name: rec.name,
        r#type: rec.r#type,
    }))
}
