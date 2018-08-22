use std::env;
use std::fs;
use std::process;

struct Config {
    path: String,
    query: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        if args.len() > 3 {
            return Err("Too many arguments, are you a lawyer?");
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    let contents = fs::read_to_string(&config.path).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("Query: {}", config.query);
    println!("In file: {}", config.path);
    println!("With text: {}", contents);
}
