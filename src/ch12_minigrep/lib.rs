use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub case_sensitive: bool,
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let query = args[2].clone();
        let filename = args[3].clone();

        Ok(Self {
            case_sensitive,
            filename,
            query,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_returns_config_object() -> Result<(), String> {
        let args = vec![
            String::from("test"),
            String::from("minigrep"),
            String::from("the"),
            String::from("poem.txt"),
        ];
        let result = Config::new(args.as_slice());

        assert!(result.is_ok());

        let config = result?;

        assert_eq!(config.query, "the");
        assert_eq!(config.filename, "poem.txt");

        Ok(())
    }

    #[test]
    fn config_returns_message_with_invalid_arguments() -> Result<(), String> {
        let args = vec![
            String::from("test"),
            String::from("minigrep"),
            String::from("the"),
        ];
        let result = Config::new(args.as_slice());

        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err, "not enough arguments");

            return Ok(());
        }

        Err(String::from("not ok"))
    }

    #[test]
    fn run_works_when_file_exists() -> Result<(), String> {
        let config = Config {
            case_sensitive: false,
            filename: String::from("poem.txt"),
            query: String::from(""),
        };
        let result = run(config);

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn run_returns_message_when_file_doesnt_exist() -> Result<(), String> {
        let config = Config {
            case_sensitive: false,
            filename: String::from(""),
            query: String::from(""),
        };
        let result = run(config);

        assert!(result.is_err());

        if let Err(err) = result {
            assert_eq!(err.description(), "entity not found");

            return Ok(());
        }

        Err(String::from("not ok"))
    }

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
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
