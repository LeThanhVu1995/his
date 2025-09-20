// iam-service error.rs placeholder
use app_error::AppError;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug)]
pub struct IamError(pub AppError);

impl std::fmt::Display for IamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AppError> for IamError {
    fn from(err: AppError) -> Self {
        IamError(err)
    }
}

impl ResponseError for IamError {
    fn error_response(&self) -> HttpResponse {
        app_web::responders::json_problem::JsonProblem::from_error(&self.0)
    }
}
