use actix_web::{HttpResponse, web};
use sqlx::{Pool, Postgres};

static PERMISSIONS_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_permissions_registered(registered: bool) {
    PERMISSIONS_REGISTERED.store(registered, std::sync::atomic::Ordering::Relaxed);
}

pub async fn healthz(db: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let mut healthy = true;
    let mut database_status = "ok".to_string();
    let mut opensearch_status = "ok".to_string();
    let mut kafka_status = "unknown".to_string();

    if let Err(e) = db.acquire().await {
        tracing::error!("Database connection error: {:?}", e);
        database_status = format!("error: {}", e);
        healthy = false;
    }

    // OpenSearch ping (best-effort)
    if let Err(e) = crate::infra::opensearch::client::OsClient::from_env().ping().await {
        tracing::warn!(error=?e, "OpenSearch not ready");
        opensearch_status = format!("error: {}", e);
        healthy = false;
    }

    // Kafka readiness (best-effort TCP check)
    if let Ok(brokers) = std::env::var("KAFKA_BROKERS") {
        let first = brokers.split(',').next().unwrap_or("").trim();
        if let Some((host, port)) = first.split_once(':') {
            match tokio::time::timeout(
                std::time::Duration::from_millis(500),
                tokio::net::TcpStream::connect((host, port.parse::<u16>().unwrap_or(9092)))
            ).await {
                Ok(Ok(_)) => kafka_status = "ok".into(),
                Ok(Err(e)) => { kafka_status = format!("error: {}", e); },
                Err(_) => { kafka_status = "timeout".into(); }
            }
        } else { kafka_status = "invalid KAFKA_BROKERS".into(); }
    } else {
        kafka_status = "not_configured".into();
    }

    let permissions_status = if PERMISSIONS_REGISTERED.load(std::sync::atomic::Ordering::Relaxed) { "registered" } else { "pending" };

    let response = serde_json::json!({
        "status": if healthy { "ok" } else { "error" },
        "service": "search-service",
        "database": database_status,
        "permissions": permissions_status,
        "opensearch": opensearch_status,
        "kafka": kafka_status,
    });

    if healthy { Ok(HttpResponse::Ok().json(response)) } else { Ok(HttpResponse::InternalServerError().json(response)) }
}

// permissions
