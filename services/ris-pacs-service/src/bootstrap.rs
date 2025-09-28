use crate::{config::Settings, telemetry, infra::db::pool};
use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;

pub async fn run() -> anyhow::Result<()> {
    // Initialize telemetry
    telemetry::init_telemetry()?;
    
    // Load configuration
    let settings = Arc::new(Settings::load());
    
    // Initialize database pool
    let db_pool = pool::create_pool(&settings.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    
    // Start HTTP server
    let server_host = settings.server_host.clone();
    let server_port = settings.server_port;
    
    tracing::info!("Starting RIS-PACS service on {}:{}", server_host, server_port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(settings.clone()))
            .wrap(Logger::default())
            .service(crate::http::routes::api_scope())
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await?;
    
    Ok(())
}