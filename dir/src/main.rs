use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let root_path = String::from("./");

    let path = if args.len() < 2 { Some(&root_path) } else { args.get(1) };

    let path = path.unwrap();

    let result = fs::read_dir(&path).expect(&format!("Could not read directory: {}", path));

    for entry in result {
        println!("{}", entry.unwrap().path().display());
    }

    println!("Hello, world!");
}