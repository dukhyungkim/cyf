use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct  ErrorResponse {
    #[serde(skip_serializing)]
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
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(self)
    }
}