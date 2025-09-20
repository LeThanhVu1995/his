// src/responders/json_problem.rs placeholder
use actix_web::{HttpResponse, http::header};
use app_error::{AppError, ProblemDetails};

/// Responder tiện lợi nếu bạn muốn tự tay build từ `AppError` (thay vì để Actix map tự động).
pub struct JsonProblem;

impl JsonProblem {
    pub fn from_error(err: &AppError) -> HttpResponse {
        let pd = err.to_problem_details(None, None, None);
        HttpResponse::build(actix_web::http::StatusCode::from_u16(pd.status).unwrap())
            .insert_header((header::CONTENT_TYPE, ProblemDetails::CONTENT_TYPE))
            .json(pd)
    }
}
