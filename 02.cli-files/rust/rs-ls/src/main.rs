use std::{env, fs, io};
use std::path::Path;

fn main() {
    let args : Vec<_> = env::args().collect();
    if args.contains(&String::from("-h")) {
        print_help(&args[0]);
        return;
    }

    let dir = Path::new(&args[1]);
    if !dir.is_dir() {
        println!("{}", dir.to_str().unwrap());
        return;
    }
    print_dir(dir).unwrap();
}

fn print_dir(dir :&Path) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        println!("{}", name.to_str().unwrap());
    }
    Ok(())
}

fn print_help(name: &str) {
    println!("usage: {} [file ...]", name);
    println!("  Options:");
    println!("  -h\t\tprint this message");
    println!("  -m\t\tStream output format; list files across the page, separated by commas.");
}