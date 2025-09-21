use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub service_name: String,
    pub service_port: u16,
    pub database_url: String,
    // Security
    pub security: app_config::SecurityConfig,
    // IAM
    pub iam_service_base_url: Option<String>,
    pub iam_service_token: Option<String>,
    // Kafka
    pub kafka_brokers: Option<String>,
    pub kafka_client_id: Option<String>,
}

impl Settings {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        let cfg = config::Config::builder()
            .add_source(config::Environment::default().separator("_"))
            .build()
            .expect("config build");
        cfg.try_deserialize().expect("config deserialize")
    }
}

