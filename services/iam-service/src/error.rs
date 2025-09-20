// iam-service error.rs placeholder
use app_error::AppError;
use actix_web::{HttpResponse, ResponseError};

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        app_web::responders::json_problem::JsonProblem::from_error(self)
    }
}
