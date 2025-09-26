use serde::{Deserialize, Serialize};
use app_config::SecurityConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub service_name: String,
    pub service_port: u16,
    pub database_url: String,
    pub security: SecurityConfig,
    pub iam_service_base_url: Option<String>,
    pub iam_service_token: Option<String>,
    pub kafka_brokers: Option<String>,
    pub kafka_group_id: Option<String>,
    pub workflow_max_parallel_branches: Option<usize>,
    pub workflow_default_timeout_secs: Option<u64>,
    pub workflow_retry_attempts: Option<u32>,
}

impl Settings {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let mut settings = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .expect("Failed to build config");

        settings.try_deserialize().expect("Failed to deserialize config")
    }
}
