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
    // MQTT (optional for future)
    pub mqtt_broker_host: Option<String>,
    pub mqtt_broker_port: Option<u16>,
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
