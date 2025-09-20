use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// RFC7807 Problem Details (Actix-only crate version).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    /// A URI reference that identifies the problem type.
    #[serde(rename = "type")]
    pub type_url: String,
    /// Short, human-readable summary of the problem type.
    pub title: String,
    /// HTTP status code.
    pub status: u16,
    /// Human-readable explanation specific to this occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    // Non-standard but useful fields:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<DateTime<Utc>>,

    /// For validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Value>,

    /// Extensions
    #[serde(flatten)]
    pub extensions: Map<String, Value>,
}

impl ProblemDetails {
    pub const CONTENT_TYPE: &'static str = "application/problem+json";

    pub fn new(status: StatusCode) -> Self {
        let title = status
            .canonical_reason()
            .unwrap_or("Unknown Error")
            .to_string();

        Self {
            type_url: "about:blank".to_string(),
            title,
            status: status.as_u16(),
            detail: None,
            instance: None,
            trace_id: None,
            correlation_id: None,
            occurred_at: Some(Utc::now()),
            errors: None,
            extensions: Map::new(),
        }
    }

    pub fn with_type(mut self, type_url: impl Into<String>) -> Self {
        self.type_url = type_url.into();
        self
    }
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }
    pub fn with_correlation_id(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
    pub fn with_errors(mut self, errors: Value) -> Self {
        self.errors = Some(errors);
        self
    }
    pub fn with_extension(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
}
