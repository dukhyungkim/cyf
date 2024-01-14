use diesel::{Insertable, Queryable};
use crate::dto;

#[derive(Debug, Clone, Queryable)]
pub struct Image {
    id: i32,
    pub title: String,
    pub url: String,
    pub alt_text: Option<String>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage {
    pub title: String,
    pub url: String,
    pub alt_text: Option<String>,
}

impl From<dto::Image> for NewImage {
    fn from(value: dto::Image) -> Self {
        Self {
            title: value.title,
            url: value.url,
            alt_text: Some(value.alt_text),
        }
    }
}
