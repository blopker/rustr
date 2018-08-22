use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, believe in yourself.");
        }
        if args.len() > 3 {
            return Err("Too many arguments, are you a lawyer?");
        }
        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    if config.case_sensitive {
        for line in search(&config.query, &contents) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    Vec::from_iter(
        contents.lines().filter(|x| {
            x.contains(query)
        })
    )
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    Vec::from_iter(
        contents.lines().filter(|x| {
            x.to_lowercase().contains(&query)
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
