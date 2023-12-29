use std::error::Error;
use crate::entity::NameScore;

pub fn parse(file: Vec<u8>) -> Result<Vec<NameScore>, Box<dyn Error>> {
    let name_scores: Vec<NameScore> = serde_json::from_slice(&file)?;
    Ok(name_scores)
}

pub fn parse_repeated(file: Vec<u8>) -> Result<Vec<NameScore>, Box<dyn Error>> {
    let mut name_scores = Vec::new();

    let contents = String::from_utf8(file)?;
    let rows: Vec<_> = contents.split("\n").collect();
    for row in rows {
        if row.is_empty() || row.starts_with("#") {
            continue;
        }

        let ns: NameScore = serde_json::from_str(row)?;
        name_scores.push(ns);
    }

    Ok(name_scores)
}
