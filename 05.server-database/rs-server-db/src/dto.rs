use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    title: String,
    url: String,
    alt_text: String,
}

impl From<entity::Image> for Image {
    fn from(value: entity::Image) -> Self {
        Self {
            title: value.title,
            url: value.url,
            alt_text: value.alt_text.unwrap_or("".to_string()),
        }
    }
}