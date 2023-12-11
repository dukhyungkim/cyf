use std::error::Error;
use std::mem;

use crate::entity::NameScore;

pub fn parse_binary(file: Vec<u8>) -> Result<Vec<NameScore>, Box<dyn Error>> {
    const BIG_ENDIAN: [u8; 2] = [0xFE, 0xFF];
    let endian = &file[..2];

    let contents = file[2..].to_vec();
    if endian == BIG_ENDIAN {
        parse_binary_fn(contents, i32::from_be_bytes)
    } else {
        parse_binary_fn(contents, i32::from_le_bytes)
    }
}

const SIZE_OF_I32: usize = mem::size_of::<i32>();

fn parse_binary_fn(contents: Vec<u8>, i32_parser: fn([u8; SIZE_OF_I32]) -> i32) -> Result<Vec<NameScore>, Box<dyn Error>> {
    let mut name_scores = Vec::new();

    let mut idx = 0;
    while idx < contents.len() {
        let score = i32_parser(contents[idx..idx + SIZE_OF_I32].try_into().unwrap());
        idx += SIZE_OF_I32;

        let name = bytes_to_string(&contents[idx..])?;
        idx += name.len() + 1;

        let ns = NameScore::new(name, score);
        name_scores.push(ns);
    }

    Ok(name_scores)
}

fn bytes_to_string(bytes: &[u8]) -> Result<String, Box<dyn Error>> {
    let null_index = bytes.iter().position(|&v| v == 0).unwrap();
    return Ok(String::from_utf8(bytes[..null_index].to_vec())?);
}