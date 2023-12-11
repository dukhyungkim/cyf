use std::error::Error;
use std::str::FromStr;

use crate::entity::NameScore;

pub fn parse(file: Vec<u8>) -> Result<Vec<NameScore>, Box<dyn Error>> {
    const NAME_INDEX: usize = 0;
    const SCORE_INDEX: usize = 1;
    const MIN_COLUMN_COUNT: usize = 2;

    let contents = String::from_utf8(file).unwrap();
    let rows: Vec<_> = contents.split("\n").collect();

    let mut name_scores = Vec::with_capacity(rows.len());
    for row in &rows[1..] {
        let columns: Vec<_> = row.split(",").collect();
        if columns.len() < MIN_COLUMN_COUNT {
            continue;
        }

        let name = columns[NAME_INDEX];
        let score = i32::from_str(columns[SCORE_INDEX]).unwrap();

        let ns = NameScore::new(name.to_string(), score);
        name_scores.push(ns);
    }
    Ok(name_scores)
}
