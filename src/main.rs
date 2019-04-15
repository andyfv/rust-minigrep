use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    print!("With text:\n{}", contents);
}

//#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename= args[2].clone();

        Config { query, filename }
    }
}
