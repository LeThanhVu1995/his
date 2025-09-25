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
    // S3
    pub s3_endpoint: Option<String>,
    pub s3_region: Option<String>,
    pub s3_bucket: Option<String>,
    pub s3_access_key: Option<String>,
    pub s3_secret_key: Option<String>,
    pub s3_use_path_style: Option<bool>,
    pub presign_expires_secs: Option<u32>,
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
