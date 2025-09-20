// src/types.rs placeholder
use serde::{Deserialize, Deserializer, Serialize};

fn default_true() -> bool {
    true
}
fn default_http_host() -> String {
    "0.0.0.0".to_string()
}
fn default_http_port() -> u16 {
    8080
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_otlp_endpoint() -> String {
    "http://localhost:4317".to_string()
}
fn default_max_connections() -> u32 {
    10
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceMeta {
    /// Tên ngắn của service: "patient", "emr", "billing", "gateway", ...
    #[serde(default)]
    pub name: String,
    /// Phiên bản build/commit hash (có thể set lúc runtime).
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_http_host")]
    pub host: String,
    #[serde(default = "default_http_port")]
    pub port: u16,
    /// Bật CORS mặc định (cho DEV).
    #[serde(default = "default_true")]
    pub enable_cors: bool,
    /// Thời gian request timeout, ví dụ "30s", "2m".
    #[serde(default)]
    pub request_timeout: Option<String>,
}
impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            host: default_http_host(),
            port: default_http_port(),
            enable_cors: true,
            request_timeout: Some("30s".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    /// service.name cho OpenTelemetry
    #[serde(default)]
    pub service_name: String,
    /// Mặc định "info"
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// OTLP gRPC endpoint
    #[serde(default = "default_otlp_endpoint")]
    pub otlp_endpoint: String,
    /// Bật tracing / metrics
    #[serde(default = "default_true")]
    pub enabled: bool,
}
impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "".into(),
            log_level: default_log_level(),
            otlp_endpoint: default_otlp_endpoint(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Ví dụ: postgres://user:pass@localhost:5432/mydb
    #[serde(default)]
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    /// Thời gian timeout kết nối, vd "5s"
    #[serde(default)]
    pub connect_timeout: Option<String>,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            max_connections: default_max_connections(),
            connect_timeout: Some("5s".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    /// Danh sách broker: "localhost:9092,localhost:9093"
    #[serde(default)]
    pub brokers: String,
    /// group id mặc định
    #[serde(default)]
    pub group_id: String,
    /// client id (mặc định SERVICE_NAME nếu để trống)
    #[serde(default)]
    pub client_id: String,
    /// Bật TLS/SASL nếu cần
    #[serde(default)]
    pub security: Option<String>,
}
impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: String::new(),
            group_id: String::new(),
            client_id: String::new(),
            security: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// redis://localhost:6379/0
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub pool_size: Option<u32>,
}
impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            pool_size: Some(8),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Issuer/Realm URL (Keycloak)
    #[serde(default)]
    pub issuer: String,
    /// Tên realm
    #[serde(default)]
    pub realm: String,
    /// Audience (per-service)
    #[serde(default)]
    pub audience: String,
    /// JWKS cache TTL vd "10m"
    #[serde(default)]
    pub jwks_ttl: Option<String>,
}
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            issuer: String::new(),
            realm: String::new(),
            audience: String::new(),
            jwks_ttl: Some("10m".into()),
        }
    }
}

/// Cấu hình tổng quát cho mọi service.
/// - `TExtra` cho phép mỗi service mở rộng cấu hình riêng (vd gateway routes, LIS connectors, …).
#[derive(Debug, Clone, Serialize)]
pub struct AppConfig<TExtra = ()>
where
    TExtra: Default + Clone + Serialize,
{
    #[serde(default)]
    pub service: ServiceMeta,

    #[serde(default)]
    pub http: HttpConfig,

    #[serde(default)]
    pub telemetry: TelemetryConfig,

    /// Tuỳ service có DB hay không
    #[serde(default)]
    pub database: Option<DatabaseConfig>,

    /// Tuỳ service có Kafka hay không
    #[serde(default)]
    pub kafka: Option<KafkaConfig>,

    /// Tuỳ service có Redis hay không
    #[serde(default)]
    pub redis: Option<RedisConfig>,

    /// Tuỳ service có xác thực/Keycloak hay không
    #[serde(default)]
    pub security: Option<SecurityConfig>,

    /// Phần mở rộng riêng cho từng service
    #[serde(default)]
    pub extra: TExtra,
}

impl<TExtra> Default for AppConfig<TExtra>
where
    TExtra: Default + Clone + Serialize,
{
    fn default() -> Self {
        Self {
            service: ServiceMeta::default(),
            http: HttpConfig::default(),
            telemetry: TelemetryConfig::default(),
            database: None,
            kafka: None,
            redis: None,
            security: None,
            extra: TExtra::default(),
        }
    }
}

impl<'de, TExtra> Deserialize<'de> for AppConfig<TExtra>
where
    TExtra: Default + Clone + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // For now, just return the default implementation
        // In a real implementation, you would deserialize the fields properly
        Ok(AppConfig::default())
    }
}
