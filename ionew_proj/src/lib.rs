use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query: query,
            filename: filename ,
            case_sensitive: case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if (config.case_sensitive) {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "Rust, false, productive.";

        assert_eq!(
            vec!["Rust, false, productive."],
            search(query,contents)
        );
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    println!("kankan:::::{:?}",results);
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}