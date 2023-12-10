use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct NameScore {
    pub name: String,
    pub high_score: i32,
}

impl NameScore {
    pub fn new(name: String, high_score: i32) -> Self {
        Self { name, high_score }
    }
}

impl Eq for NameScore {}

impl PartialEq<Self> for NameScore {
    fn eq(&self, other: &Self) -> bool {
        self.high_score == other.high_score
    }
}

impl PartialOrd<Self> for NameScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NameScore {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.high_score.cmp(&other.high_score);
    }
}
