use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    })
}

struct Config {
    filename: String,
    search_type: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let search_type = args[2].clone();

        Ok(
            Config{
                filename,
                search_type,
            }
        )
    }
}
