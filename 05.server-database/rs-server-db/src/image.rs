use std::string::ToString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    title: String,
    alt_text: String,
    url: String,
}

pub fn images() -> Vec<Image> {
    vec!(
        Image {
            title: "Sunset".to_string(),
            alt_text: "Clouds at sunset".to_string(),
            url: "https://images.unsplash.com/photo-1506815444479-bfdb1e96c566?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1000&q=80".to_string(),
        },
        Image {
            title: "Mountain".to_string(),
            alt_text: "A mountain at sunset".to_string(),
            url: "https://images.unsplash.com/photo-1540979388789-6cee28a1cdc9?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1000&q=80".to_string(),
        }
    )
}
