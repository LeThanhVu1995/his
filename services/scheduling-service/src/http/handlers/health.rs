use actix_web::{HttpResponse, web};
use sqlx::{Pool, Postgres};
use crate::infrastructure::iam_client;
use crate::config::Settings;

static mut PERMISSIONS_REGISTERED: bool = false;

pub fn set_permissions_registered(status: bool) {
    unsafe {
        PERMISSIONS_REGISTERED = status;
    }
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    database: String,
    iam_registration: String,
}

pub async fn healthz(
    cfg: web::Data<Settings>,
    db: web::Data<Pool<Postgres>>,
) -> actix_web::Result<HttpResponse> {
    let mut healthy = true;
    let mut database_status = "ok".to_string();
    let mut iam_status = "ok".to_string();

    // Check database connection
    if let Err(e) = db.acquire().await {
        tracing::error!("Database connection error: {:?}", e);
        database_status = format!("error: {}", e);
        healthy = false;
    }

    // Check IAM service registration status
    if unsafe { !PERMISSIONS_REGISTERED } {
        if let Err(e) = iam_client::register_permissions(&cfg).await {
            tracing::error!("IAM permission registration failed: {:?}", e);
            iam_status = format!("error: {}", e);
            healthy = false;
        } else {
            set_permissions_registered(true);
        }
    }

    let response = HealthResponse {
        status: if healthy { "ok".to_string() } else { "error".to_string() },
        service: cfg.service_name.clone(),
        database: database_status,
        iam_registration: iam_status,
    };

    if healthy {
        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::InternalServerError().json(response))
    }
}
