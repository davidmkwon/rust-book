use std::env;
use std::error::Error;
use std::fs;

use rand::{thread_rng, Rng};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // checks that there are sufficient arguments to make a Config
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg (just name of program)
        args.next();

        // iterate to get query and filename
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("couldn't extract query"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("couldn't extract filename"),
        };

        Ok(Config {
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            query,
            filename,
        })
    }

    // checks that there are sufficient arguments to make a Config
    pub fn new_random(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg (just name of program)
        args.next();

        // iterate to get query and filename
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("couldn't extract query"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("couldn't extract filename"),
        };

        let mut rng = thread_rng();
        Ok(Config {
            case_sensitive: rng.gen_bool(0.5),
            query,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.filename)?;

    // get results of search
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // print results out
    for result in results {
        println!("{}", result);
    }

    Ok(())
}

// returns the lines in `contents` that contain `query`
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// returns the lines in `contents` that contain `query` without considering cases
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust Me.
Duct tape.";

        assert_eq!(
            vec!["Rust:", "Trust Me."],
            search_case_insensitive(query, contents)
        );
    }
}
