// src/loader.rs placeholder
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Serialize};
use std::{env, path::PathBuf};
use thiserror::Error;

use crate::types::{AppConfig, TelemetryConfig};

#[derive(Debug, Clone, Copy)]
pub enum AppEnv {
    Dev,
    Prod,
    Test,
}

impl AppEnv {
    pub fn from_env() -> Self {
        match env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string()).to_lowercase().as_str() {
            "prod" | "production" => Self::Prod,
            "test" => Self::Test,
            _ => Self::Dev,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Prod => "prod",
            Self::Test => "test",
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    File(String),
    #[error("Invalid configuration: {0}")]
    Invalid(String),
    #[error("Serialization error: {0}")]
    Serde(String),
}

static DEFAULT_CONFIG_DIR: Lazy<String> = Lazy::new(|| {
    env::var("CONFIG_DIR").unwrap_or_else(|_| "config".to_string())
});

fn config_file_path(env: AppEnv, service_file_stem: &str) -> PathBuf {
    let mut p = PathBuf::from(&*DEFAULT_CONFIG_DIR);
    p.push(env.as_str());
    p.push(format!("{service_file_stem}.yaml"));
    p
}

pub fn load<TExtra>(
    service_file_stem: &str,
    service_name_fallback: &str,
) -> Result<AppConfig<TExtra>, ConfigError>
where
    TExtra: Default + Clone + Serialize + DeserializeOwned,
{
    load_with::<TExtra>(service_file_stem, service_name_fallback, AppEnv::from_env())
}

pub fn load_with<TExtra>(
    service_file_stem: &str,
    service_name_fallback: &str,
    env_kind: AppEnv,
) -> Result<AppConfig<TExtra>, ConfigError>
where
    TExtra: Default + Clone + Serialize + DeserializeOwned,
{
    let base = AppConfig::<TExtra>::default();

    let file_path = config_file_path(env_kind, service_file_stem);
    let yaml_provider = Yaml::file(&file_path);

    let service_prefix = service_file_stem.to_uppercase();

    let figment = Figment::from(Serialized::defaults(base))
        .merge(yaml_provider)
        .merge(Env::prefixed("APP__").split("__"))
        .merge(Env::prefixed(&format!("{service_prefix}__")).split("__"));

    let mut cfg: AppConfig<TExtra> = figment
        .extract()
        .map_err(|e| ConfigError::Invalid(e.to_string()))?;

    if cfg.telemetry.service_name.trim().is_empty() {
        cfg.telemetry.service_name = service_name_fallback.to_string();
    }

    normalize_telemetry(&mut cfg.telemetry);

    Ok(cfg)
}

fn normalize_telemetry(t: &mut TelemetryConfig) {
    if t.log_level.trim().is_empty() {
        t.log_level = "info".into();
    }
    if t.otlp_endpoint.trim().is_empty() {
        t.otlp_endpoint = "http://localhost:4317".into();
    }
}
