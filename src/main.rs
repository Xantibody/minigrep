use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    // {}を探しています
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1].clone();
    let filename = &args[2].clone();

    Config(query, filename)
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読込中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("With text:\n{}", contents);
}
