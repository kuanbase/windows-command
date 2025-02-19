use std::fs;
use std::process;

fn main() {

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        usage();
    }

    let path = args.get(1);

    let path = path.unwrap();

    let result = fs::read_dir(&path).expect(&format!("Could not read directory: {}", path));

    for entry in result {
        println!("{}", entry.unwrap().path().display());
    }

    println!("Hello, world!");
}

fn usage() {
    println!("Usage: dir [<empty> | <path>]");
    process::exit(1);
}