use std::{error::Error, fs::read_to_string};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(&config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Very few arguements!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    let query= &query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let query= "rob";
        let content = "\
Rusty Rustaceans
rapidly refactor robust
Rust routines.";

        assert_eq!(vec!["rapidly refactor robust"], search(&query, &content));

    }

    #[test]
    fn test_two() {
        let query= "roB";
        let content = "\
Rusty Rustaceans
rapidly refactor robust
Rust routines.";

        assert_eq!(vec!["rapidly refactor robust"], search(&query, &content));

    }
}
