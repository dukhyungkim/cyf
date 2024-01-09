use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Image {
    id: i32,
    title: String,
    url: String,
    alt_text: Option<String>,
}
