use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
