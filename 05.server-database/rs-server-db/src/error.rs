use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    status_code: StatusCode,
    message: String,
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

    pub fn invalid_image_url() -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: "invalid image url".to_string(),
        }
    }

    pub fn internal_server_error(message: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl ErrorResponse {
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(self)
    }
}

impl From<reqwest::Error> for ErrorResponse {
    fn from(err: reqwest::Error) -> Self {
        let message = format!("failed to send request: {}", err);
        Self::internal_server_error(message)
    }
}

impl From<reqwest::header::ToStrError> for ErrorResponse {
    fn from(err: reqwest::header::ToStrError) -> Self {
        let message = format!("failed to convert header to string: {}", err);
        Self::internal_server_error(message)
    }
}
