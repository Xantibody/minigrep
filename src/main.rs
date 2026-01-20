use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);

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
}
