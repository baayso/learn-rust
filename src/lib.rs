use std::error::Error;
use std::{env, fs};

pub struct Argument {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Argument {
    pub fn new(mut args: std::env::Args) -> Result<Argument, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Argument {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(argument: Argument) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(argument.filename)?;

    let result_list = if argument.case_sensitive {
        search(&argument.query, &contents)
    } else {
        search_case_insensitive(&argument.query, &contents)
    };

    for line in result_list {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// search_ignore_case
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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // ignore_case
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
