use serde::Deserialize;
use app_config::SecurityConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub service_name: String,
    pub service_port: u16,
    pub database_url: String,
    // Security
    pub security: SecurityConfig,
    // IAM
    pub iam_service_base_url: Option<String>,
    pub iam_service_token: Option<String>,
    // Kafka
    pub kafka_brokers: Option<String>,
    pub audit_topic: Option<String>,
    pub allow_direct_write: Option<bool>,
}

impl Settings {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
