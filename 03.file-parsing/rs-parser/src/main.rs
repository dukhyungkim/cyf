use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use rs_parser::entity::NameScore;
use rs_parser::parser::Parser;

fn main() {
    run_app().unwrap();
}

fn run_app() -> Result<(), Box<dyn Error>> {
    let asset_dir = Path::new("../../assets");

    let list = HashMap::from([
        ("custom-binary-be.bin", Parser::BinaryParser),
        ("custom-binary-le.bin", Parser::BinaryParser),
        ("data.csv", Parser::CSVParser),
        ("json.txt", Parser::JSONParser),
        ("repeated-json.txt", Parser::RepeatedJSONParser),
    ]);
    for (filename, parser) in list {
        let filepath = asset_dir.join(filename);
        let file = fs::read(filepath)?;

        let name_scores = parser.parse(file)?;

        println!("read {} and results:", filename);
        print_result(&name_scores);
        println!();
    }
    Ok(())
}

fn print_result(name_scores: &Vec<NameScore>) {
    let highest = name_scores.iter().max().unwrap();
    let lowest = name_scores.iter().min().unwrap();

    println!("highest => name: {}, score: {}", highest.name, highest.high_score);
    println!("lowest => name: {}, score: {}", lowest.name, lowest.high_score);
}
