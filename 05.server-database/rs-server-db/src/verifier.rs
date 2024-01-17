use actix_web::web::Data;
use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;

use crate::database::Database;
use crate::entity;
use crate::error::ErrorResponse;

pub async fn verify_image(db: &Data<Database>, image: &entity::NewImage) -> Result<(), ErrorResponse> {
    let alt_text = &image.alt_text;
    match alt_text {
        Some(alt_text) => {
            if alt_text.is_empty() {
                return Err(ErrorResponse::invalid_alt_text());
            }
        }
        None => return Err(ErrorResponse::invalid_alt_text())
    }

    if db.is_duplicated_image(image) {
        return Err(ErrorResponse::duplicate_error());
    }


    if let Err(err) = verify_url(&image.url).await {
        return Err(err);
    }

    Ok(())
}

async fn verify_url(url: &str) -> Result<(), ErrorResponse> {
    let response = reqwest::Client::new().head(url).send().await?;

    if response.status() != StatusCode::OK {
        return Err(ErrorResponse::invalid_image_url());
    }

    if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
        let content_type_str = content_type.to_str()?;
        if content_type_str.starts_with("image/") {
            return Ok(());
        }
    }

    return Err(ErrorResponse::invalid_image_url());
}
