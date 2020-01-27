use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {

        if args.len() < 3 {
            return Err(String::from("Fuck you!"));
        }

        Ok(Self {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_name)?;

    for line in search(&config.query, &contents[..]) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_test() {
        let query = "fuckfuck";
        let contents = "\
            Rust
            fuckfuck
            Do you like van you xi.";

        assert_eq!(
            vec!["            fuckfuck"],
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