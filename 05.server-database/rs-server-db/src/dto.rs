use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Debug, Deserialize)]
pub struct ImageRequest {
    pub indent: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub title: String,
    pub url: String,
    pub alt_text: String,
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
