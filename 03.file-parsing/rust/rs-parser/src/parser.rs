use std::error::Error;

use crate::{binary, csv, json};
use crate::entity::NameScore;

pub enum Parser {
    BinaryParser,
    CSVParser,
    JSONParser,
    RepeatedJSONParser,
}

impl Parser {
    pub fn parse(&self, file: Vec<u8>) -> Result<Vec<NameScore>, Box<dyn Error>> {
        match self {
            Parser::BinaryParser => binary::parse(file),
            Parser::CSVParser => csv::parse(file),
            Parser::JSONParser => json::parse(file),
            Parser::RepeatedJSONParser => json::parse_repeated(file)
        }
    }
}
