use diesel::{Insertable, Queryable};

#[derive(Debug, Clone, Queryable)]
pub struct Image {
    id: i32,
    pub title: String,
    pub url: String,
    pub alt_text: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::images)]
pub struct NewImage {
    pub title: String,
    pub url: String,
    pub alt_text: Option<String>,
}