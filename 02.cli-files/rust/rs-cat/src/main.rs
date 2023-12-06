use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.contains(&String::from("-h")) {
        print_help(&args[0]);
        return;
    }

    let mut files = Vec::new();
    let mut print_line_num = false;
    for arg in &args[1..] {
        let arg = arg.as_str();
        if arg == "-n" {
            print_line_num = true;
        }
        if arg.starts_with("-") {
            continue;
        }
        files.push(arg);
    }

    for file in files {
        let contents = fs::read_to_string(file).unwrap();

        if print_line_num {
            print_with_line_num(&contents);
            continue;
        }
        print!("{}", contents);
    }
}

fn print_with_line_num(contents: &str) {
    let lines: Vec<_> = contents.split("\n").collect();
    let last = lines.len() -1;
    for (i, line) in lines.iter().enumerate() {
        print!("{:6}\t{}", i+1, line);
        if i != last {
            println!()
        }
    }
}

fn print_help(name: &str) {
    println!("usage: {} [file ...]", name);
    println!("  Options:");
    println!("  -h\t\tprint this message");
    println!("  -m\t\tNumber the output lines, starting at 1");
}
