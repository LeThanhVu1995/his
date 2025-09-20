// iam-service config types.rs placeholder
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub service_name: String,
    pub service_version: String,
    pub http_host: String,
    pub http_port: u16,
    pub dev_mode: bool,

    pub database_url: String,

    pub telemetry: app_config::TelemetryConfig,
    pub security: app_config::SecurityConfig,
    pub kafka: app_config::KafkaConfig,
    // login helpers
    pub oidc_login_client_id: String,
    pub idp_providers: Vec<String>,
}
