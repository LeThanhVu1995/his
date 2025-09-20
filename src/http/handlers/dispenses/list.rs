use actix_web::{get, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::DispenseRepo;
use crate::http::dto::dispense_dto::{DispenseQuery, DispenseRes};

#[get("/api/v1/dispenses")]
pub async fn list_dispenses(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<DispenseQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = DispenseRepo { db: &db };
    let (items, total) = repo
        .list_paged(q.prescription_id, page, size)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list disp");
            crate::error::AppError::Internal("DB".into())
        })?;
    let res: Vec<DispenseRes> = items
        .into_iter()
        .map(|d| DispenseRes {
            id: d.id,
            disp_no: d.disp_no,
            status: d.status,
        })
        .collect();
    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}
