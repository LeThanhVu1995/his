// src/result_ext.rs placeholder
//! Tiện ích chuyển đổi lỗi `sqlx` thành dạng dễ dùng với `app-error`.

use app_error::AppError;
use sqlx::Error as SqlxError;

/// Mở rộng cho `Result<T, sqlx::Error>`.
pub trait ResultExt<T> {
    /// Map `RowNotFound` → `AppError::NotFound { resource, id }`.
    fn or_not_found(self, resource: &str, id: Option<String>) -> Result<T, AppError>;

    /// Chuyển `RowNotFound` → `Ok(None)`, lỗi khác -> `Err(AppError)`.
    fn not_found_to_none(self) -> Result<Option<T>, AppError>;
}

impl<T> ResultExt<T> for Result<T, SqlxError> {
    fn or_not_found(self, resource: &str, id: Option<String>) -> Result<T, AppError> {
        match self {
            Ok(v) => Ok(v),
            Err(SqlxError::RowNotFound) => Err(AppError::NotFound {
                resource: resource.to_string(),
                id,
            }),
            Err(e) => Err(AppError::from(e)),
        }
    }

    fn not_found_to_none(self) -> Result<Option<T>, AppError> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(SqlxError::RowNotFound) => Ok(None),
            Err(e) => Err(AppError::from(e)),
        }
    }
}

/// Mở rộng cho `Option<T>` khi muốn ép phải có giá trị.
pub trait OptionExt<T> {
    /// Nếu `None` → `AppError::NotFound { resource, id }`.
    fn or_not_found(self, resource: &str, id: Option<String>) -> Result<T, AppError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_not_found(self, resource: &str, id: Option<String>) -> Result<T, AppError> {
        self.ok_or_else(|| AppError::NotFound {
            resource: resource.to_string(),
            id,
        })
    }
}

/* -------------------- Tests -------------------- */
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn option_or_not_found() {
        let v: Option<i32> = None;
        let e = v.or_not_found("patient", Some("p1".into())).unwrap_err();
        match e {
            AppError::NotFound { resource, id } => {
                assert_eq!(resource, "patient");
                assert_eq!(id.as_deref(), Some("p1"));
            }
            _ => panic!("wrong error"),
        }
    }
}
