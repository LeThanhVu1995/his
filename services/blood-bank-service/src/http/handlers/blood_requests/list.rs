use actix_web::{get, web, HttpResponse, Responder};
use crate::http::dto::blood_request_dto::{ListBloodRequestsQuery, ListBloodRequestsResponse};

#[get("/api/v1/blood/requests")]
pub async fn list_blood_requests(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<ListBloodRequestsQuery>,
) -> impl Responder {
    let _blood_request_repo = crate::infra::db::repositories::blood_request_repo::BloodRequestRepo { db: &db };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    // For now, return empty list - in real implementation would filter by query params
    let requests = vec![]; // TODO: Implement filtering in repository

    HttpResponse::Ok().json(ListBloodRequestsResponse {
        total: requests.len(),
        requests,
        limit,
        offset,
    })
}
