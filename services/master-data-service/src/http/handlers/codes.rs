use actix_web::{web, HttpResponse};
use uuid::Uuid;
use actix_web_validator::{Query, Json};
use actix_web::web::Path;
use app_web::extractors::auth_user::AuthUser;

use crate::{domain::{repo::MasterRepo, service::MasterService}, dto::{code_dto::{CreateCodeReq, UpdateCodeReq, CodeRes, ListCodesQuery, calc_etag, BulkCreateCodeReq, BulkUpdateCodeReq, BulkCreateCodeRes, BulkError}}, infrastructure::kafka::Kafka};

#[utoipa::path(
    get,
    path="/api/v1/master/codes",
    security(("bearer_auth" = [])),
    params(("category" = Option<String>, Query, description = "Filter by category")),
    responses(
        (status=200, description="List codes",
          body = [CodeRes])
    )
)]
pub async fn list_codes(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<ListCodesQuery>,
    _user: AuthUser
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = MasterRepo { db: &db };

    // Use search if search term or is_active filter is provided
    let (items, total) = if q.search.is_some() || q.is_active.is_some() {
        repo.search_codes_paged(
            q.category.as_deref(),
            q.search.as_deref(),
            q.is_active,
            page,
            size
        ).await.map_err(|e| {
            tracing::error!(error=?e, "search_codes");
            crate::error::AppError::Internal("DB".into())
        })?
    } else {
        repo.list_codes_paged(q.category.as_deref(), page, size).await.map_err(|e| {
            tracing::error!(error=?e, "list_codes");
            crate::error::AppError::Internal("DB".into())
        })?
    };

    let res: Vec<CodeRes> = items.into_iter().map(|m| CodeRes {
        id: m.id, category: m.category, code: m.code, name: m.name, description: m.description, is_active: m.is_active,
    }).collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);

    // If-None-Match
    if let Some(tag) = req.headers().get(actix_web::http::header::IF_NONE_MATCH).and_then(|h| h.to_str().ok()) {
        if tag == etag { return Ok(HttpResponse::NotModified().finish()); }
    }

    Ok(HttpResponse::Ok()
        .append_header((actix_web::http::header::ETAG, etag))
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .content_type("application/json")
        .body(body))
}

#[utoipa::path(
    post,
    path="/api/v1/master/codes:create",
    security(("bearer_auth" = [])),
    request_body=CreateCodeReq,
    responses(
        (status=201, description="Created", body = CodeRes)
    )
)]
pub async fn create_code(db: web::Data<sqlx::Pool<sqlx::Postgres>>, kafka: web::Data<Option<Kafka>>, payload: Json<CreateCodeReq>) -> actix_web::Result<HttpResponse> {
    let repo = MasterRepo { db: &db };
    let svc = MasterService { repo, kafka: kafka.get_ref().as_ref() };
    let id = svc.create_code(&payload.category, &payload.code, &payload.name, payload.description.as_deref())
        .await
        .map_err(|e| { tracing::error!(?e, "create_code"); crate::error::AppError::Internal("DB".into()) })?;

    // quay lại bản ghi để trả response
    let repo = MasterRepo { db: &db };
    let rec = repo.find_code(id).await.map_err(|e| { tracing::error!(?e, "get created"); crate::error::AppError::Internal("DB".into()) })?
        .ok_or_else(|| crate::error::AppError::NotFound)?;

    let res = CodeRes { id: rec.id, category: rec.category, code: rec.code, name: rec.name, description: rec.description, is_active: rec.is_active };
    Ok(HttpResponse::Created().json(res))
}

#[utoipa::path(
    put,
    path="/api/v1/master/codes/{id}",
    security(("bearer_auth" = [])),
    request_body=UpdateCodeReq,
    responses(
        (status=200, description="Updated", body = CodeRes)
    )
)]
pub async fn update_code(db: web::Data<sqlx::Pool<sqlx::Postgres>>, kafka: web::Data<Option<Kafka>>, path: Path<Uuid>, payload: Json<UpdateCodeReq>) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = MasterRepo { db: &db };
    let svc = MasterService { repo, kafka: kafka.get_ref().as_ref() };
    let ok = svc.update_code(id, payload.name.as_deref(), payload.description.as_deref(), payload.is_active)
        .await
        .map_err(|e| { tracing::error!(?e, "update_code"); crate::error::AppError::Internal("DB".into()) })?;

    if !ok { return Err(crate::error::AppError::NotFound.into()); }

    let repo = MasterRepo { db: &db };
    let rec = repo.find_code(id).await.map_err(|e| { tracing::error!(?e, "get updated"); crate::error::AppError::Internal("DB".into()) })?
        .ok_or_else(|| crate::error::AppError::NotFound)?;

    let res = CodeRes { id: rec.id, category: rec.category, code: rec.code, name: rec.name, description: rec.description, is_active: rec.is_active };
    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    delete,
    path="/api/v1/master/codes/{id}",
    security(("bearer_auth" = [])),
    responses((status=204, description="Deleted"))
)]
pub async fn delete_code(db: web::Data<sqlx::Pool<sqlx::Postgres>>, path: Path<Uuid>) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = MasterRepo { db: &db };
    let rows = repo.delete_code(id).await.map_err(|e| { tracing::error!(?e, "delete_code"); crate::error::AppError::Internal("DB".into()) })?;
    if rows == 0 { return Err(crate::error::AppError::NotFound.into()); }
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    post,
    path="/api/v1/master/codes/bulk",
    security(("bearer_auth" = [])),
    request_body=BulkCreateCodeReq,
    responses(
        (status=201, description="Bulk created", body = BulkCreateCodeRes)
    )
)]
pub async fn bulk_create_codes(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    kafka: web::Data<Option<Kafka>>,
    payload: Json<BulkCreateCodeReq>
) -> actix_web::Result<HttpResponse> {
    let repo = MasterRepo { db: &db };
    let svc = MasterService { repo, kafka: kafka.get_ref().as_ref() };

    let mut created = Vec::new();
    let mut failed = Vec::new();

    for (index, code_req) in payload.codes.iter().enumerate() {
        match svc.create_code(&code_req.category, &code_req.code, &code_req.name, code_req.description.as_deref()).await {
            Ok(id) => {
                // Fetch the created record
                let repo = MasterRepo { db: &db };
                if let Ok(Some(rec)) = repo.find_code(id).await {
                    created.push(CodeRes {
                        id: rec.id,
                        category: rec.category,
                        code: rec.code,
                        name: rec.name,
                        description: rec.description,
                        is_active: rec.is_active,
                    });
                }
            }
            Err(e) => {
                failed.push(BulkError {
                    index,
                    error: e.to_string(),
                });
            }
        }
    }

    let response = BulkCreateCodeRes {
        total_created: created.len(),
        total_failed: failed.len(),
        created,
        failed,
    };

    Ok(HttpResponse::Created().json(response))
}

#[utoipa::path(
    put,
    path="/api/v1/master/codes/bulk",
    security(("bearer_auth" = [])),
    request_body=BulkUpdateCodeReq,
    responses(
        (status=200, description="Bulk updated", body = BulkCreateCodeRes)
    )
)]
pub async fn bulk_update_codes(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    kafka: web::Data<Option<Kafka>>,
    payload: Json<BulkUpdateCodeReq>
) -> actix_web::Result<HttpResponse> {
    let repo = MasterRepo { db: &db };
    let svc = MasterService { repo, kafka: kafka.get_ref().as_ref() };

    let mut updated = Vec::new();
    let mut failed = Vec::new();

    for (index, update_req) in payload.updates.iter().enumerate() {
        match svc.update_code(update_req.id, update_req.name.as_deref(), update_req.description.as_deref(), update_req.is_active).await {
            Ok(true) => {
                // Fetch the updated record
                let repo = MasterRepo { db: &db };
                if let Ok(Some(rec)) = repo.find_code(update_req.id).await {
                    updated.push(CodeRes {
                        id: rec.id,
                        category: rec.category,
                        code: rec.code,
                        name: rec.name,
                        description: rec.description,
                        is_active: rec.is_active,
                    });
                }
            }
            Ok(false) => {
                failed.push(BulkError {
                    index,
                    error: "Code not found".to_string(),
                });
            }
            Err(e) => {
                failed.push(BulkError {
                    index,
                    error: e.to_string(),
                });
            }
        }
    }

    let response = BulkCreateCodeRes {
        total_created: updated.len(),
        total_failed: failed.len(),
        created: updated,
        failed,
    };

    Ok(HttpResponse::Ok().json(response))
}
