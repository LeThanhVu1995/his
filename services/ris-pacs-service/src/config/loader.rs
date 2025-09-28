use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
    pub security: app_config::SecurityConfig,
    // IAM
    pub iam_service_base_url: Option<String>,
    pub iam_service_token: Option<String>,
    // Kafka
    pub kafka_brokers: Option<String>,
    pub kafka_client_id: Option<String>,
    // PACS
    pub pacs_ae_title: Option<String>,
    pub pacs_host: Option<String>,
    pub pacs_port: Option<u16>,
    pub storage_path: Option<String>,
}

impl Settings {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/ris_pacs".to_string()),
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            security: app_config::SecurityConfig::default(),
            iam_service_base_url: std::env::var("IAM_SERVICE_BASE_URL").ok(),
            iam_service_token: std::env::var("IAM_SERVICE_TOKEN").ok(),
            kafka_brokers: std::env::var("KAFKA_BROKERS").ok(),
            kafka_client_id: std::env::var("KAFKA_CLIENT_ID").ok(),
            pacs_ae_title: std::env::var("PACS_AE_TITLE").ok(),
            pacs_host: std::env::var("PACS_HOST").ok(),
            pacs_port: std::env::var("PACS_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            storage_path: std::env::var("STORAGE_PATH").ok(),
        }
    }
}