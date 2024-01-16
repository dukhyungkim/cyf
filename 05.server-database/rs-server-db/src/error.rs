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
            message: "duplicated image".to_string(),
        }
    }

    pub fn invalid_alt_text() -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: "invalid alt_text".to_string(),
        }
    }
}

impl ErrorResponse {
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(self)
    }
}