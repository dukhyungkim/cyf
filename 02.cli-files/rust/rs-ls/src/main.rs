use std::{env, fs, io};
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.contains(&String::from("-h")) {
        print_help(&args[0]);
        return;
    }

    let mut read_path = ".";
    let mut opts = Vec::new();
    for arg in &args[1..] {
        if arg.starts_with("-") {
            opts.push(arg.as_str());
            continue;
        }
        read_path = arg
    }

    let dir = Path::new(read_path);
    if !dir.is_dir() {
        println!("{}", dir.to_str().unwrap());
        return;
    }
    let entries = read_dir(dir).unwrap();
    print_dir(entries, opts);
}

fn read_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let filename = name.to_str().unwrap().to_string();
        entries.push(filename);
    }

    entries.sort();
    Ok(entries)
}

fn print_dir(entries: Vec<String>, opts: Vec<&str>) {
    if opts.is_empty() {
        entries.iter().for_each(|name| println!("{}", name));
        return;
    }

    if opts[0] == "-m" {
        println!("{}", entries.join(", "));
        return;
    }

    entries.iter().for_each(|name| println!("{}", name));
}

fn print_help(name: &str) {
    println!("usage: {} [file ...]", name);
    println!("  Options:");
    println!("  -h\t\tprint this message");
    println!("  -m\t\tStream output format; list files across the page, separated by commas.");
}