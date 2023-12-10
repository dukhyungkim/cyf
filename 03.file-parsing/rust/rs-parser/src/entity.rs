#[derive(Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct NameScore {
    pub name: String,
    pub high_score: i32,
}

impl NameScore {
    pub fn new(name: String, high_score: i32) -> Self {
        Self { name, high_score }
    }
}
