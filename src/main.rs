use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // {}を探しています
    println!("Searching for {}", query);
    // {}というファイルの中
    println!("In file {}", filename);

    let (query, filename) = parse_config(&args);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
