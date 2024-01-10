use diesel::Queryable;

#[derive(Debug, Clone, Queryable)]
pub struct Image {
    id: i32,
    pub title: String,
    pub url: String,
    pub alt_text: Option<String>,
}
