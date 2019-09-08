use std::env;

struct Config {
    case_sensitive: bool,
    filename: String,
    query: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next(); // Binary
        args.next(); // "minigrep" command

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Self {
            case_sensitive,
            filename,
            query,
        })
    }
}

pub fn run() {
    let config = Config::new(env::args());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
