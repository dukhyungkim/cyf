use serde::{Deserialize, Serialize};

use crate::entity;
use crate::entity::NewImage;

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

impl Into<entity::NewImage> for Image {
    fn into(self) -> NewImage {
        NewImage{
            title: self.title,
            url: self.url,
            alt_text: Some(self.alt_text),
        }
    }
}