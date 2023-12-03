use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>, // args can be any type that implements the Iterator
                                                // type and returns String items. Iterating over args
                                                // will mutate it.
    ) -> Result<Config, &'static str> {
        args.next(); // Ignoring first value ot env::args which is the name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // ? returns/throws error to the caller instead of panic
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
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
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Iterate through each line of the contents.
    // Check whether the line contains our query string.
    // If it does, add it to the list of values we’re returning.
    // If it doesn’t, do nothing.
    // Return the list of results that match.
   
    // let mut results: Vec<&str> = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }

    // results

    // Equivalently, using iterator and closure
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Allocates a new String (no longer &str)
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line.trim());
    //     }
    // }

    // results
    
    // Equivalently, using iterator and closure
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
