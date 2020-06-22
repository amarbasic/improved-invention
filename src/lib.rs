use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename")
        };

        Ok(Config { query, filename })
    }

    pub fn get_query(&self) -> &String {
        return &self.query;
    }

    pub fn get_filename(&self) -> &String {
        return &self.filename;
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_search_found() {
        let query = "monomorphization";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert!(search(query, contents).is_empty());
    }
}
