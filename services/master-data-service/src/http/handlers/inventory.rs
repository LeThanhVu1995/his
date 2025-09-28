use actix_web::{web, HttpResponse, Result};
use crate::domain::models::{
    InvUom, CreateUomRequest, UpdateUomRequest, UomQuery
};
// use crate::infra::db::repositories::inventory_repo::InvUomRepo;
use utoipa::ToSchema;

// UOM Management
#[utoipa::path(
    post,
    path = "/api/v1/master/uoms",
    request_body = CreateUomRequest,
    responses(
        (status = 201, description = "UOM created successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateUomRequest>,
) -> Result<HttpResponse> {
    let uom_id = uuid::Uuid::new_v4().to_string();

    let uom = InvUom {
        uom_id: uom_id.clone(),
        code: body.code.clone(),
        name: body.name.clone(),
    };

    // InvUomRepo { db: &db }
    //     .create(&uom)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to create UOM"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": uom_id})))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/uoms",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("code" = Option<String>, Query, description = "Filter by UOM code")
    ),
    responses(
        (status = 200, description = "List of UOMs"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_uoms(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<UomQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);
    let offset = (page - 1) * page_size;

    // let uoms = InvUomRepo { db: &db }
    //     .list_paged(query.code.clone(), page_size, offset)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?;
    let uoms: Vec<InvUom> = vec![];

    Ok(HttpResponse::Ok().json(uoms))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/uoms/{id}",
    params(
        ("id" = String, Path, description = "UOM ID")
    ),
    responses(
        (status = 200, description = "UOM found"),
        (status = 404, description = "UOM not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let uom_id = path.into_inner();

    // let uom = InvUomRepo { db: &db }
    //     .get_by_id(&uom_id)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    //     .ok_or_else(|| actix_web::error::ErrorNotFound("UOM not found"))?;
    return Err(actix_web::error::ErrorNotFound("UOM not found"));
}

#[utoipa::path(
    put,
    path = "/api/v1/master/uoms/{id}",
    params(
        ("id" = String, Path, description = "UOM ID")
    ),
    request_body = UpdateUomRequest,
    responses(
        (status = 200, description = "UOM updated successfully"),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "UOM not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
    body: web::Json<UpdateUomRequest>,
) -> Result<HttpResponse> {
    let uom_id = path.into_inner();

    // let mut uom = InvUomRepo { db: &db }
    //     .get_by_id(&uom_id)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("DB error"))?
    //     .ok_or_else(|| actix_web::error::ErrorNotFound("UOM not found"))?;

    // // Apply updates
    // if let Some(name) = body.name.clone() { uom.name = name; }

    // InvUomRepo { db: &db }
    //     .update(&uom_id, &uom)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to update UOM"))?;

    return Err(actix_web::error::ErrorNotFound("UOM not found"));
}

#[utoipa::path(
    delete,
    path = "/api/v1/master/uoms/{id}",
    params(
        ("id" = String, Path, description = "UOM ID")
    ),
    responses(
        (status = 204, description = "UOM deleted successfully"),
        (status = 404, description = "UOM not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_uom(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let uom_id = path.into_inner();

    // InvUomRepo { db: &db }
    //     .delete(&uom_id)
    //     .await
    //     .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to delete UOM"))?;

    Ok(HttpResponse::NoContent().finish())
}
