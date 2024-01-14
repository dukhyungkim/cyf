use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct  ErrorResponse {
    status_code: StatusCode,
    message: String
}

impl ErrorResponse {
    pub fn duplicate_error() -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: "duplicated".to_string(),
        }
    }
}

impl ErrorResponse {
    pub fn http_response(&self) -> impl Responder {
        HttpResponse::build(self.status_code).json(self)
    }
}