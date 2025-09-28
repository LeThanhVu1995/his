use serde::{Deserialize, Serialize};

// Common API Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: None,
        }
    }

    pub fn error_with_message(error: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            message: Some(message),
        }
    }
}

// Pagination DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        let has_next = offset + limit < total;
        let has_prev = offset > 0;

        Self {
            data,
            meta: PaginationMeta {
                total,
                limit,
                offset,
                has_next,
                has_prev,
            },
        }
    }
}

// Health check DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub service: String,
    pub version: Option<String>,
}

// Search DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// Generic list query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequest {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
