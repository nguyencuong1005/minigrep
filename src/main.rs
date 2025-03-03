use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok((Config {
            // `args` is a reference to a `Vec<String>`, and since `query` and `file_path` are of type `String` (not `&str`),
            // we need to clone the strings to take ownership without moving them out of the borrowed vector.
            query: args[1].clone(),
            file_path: args[2].clone(),
        }))
    }
}
