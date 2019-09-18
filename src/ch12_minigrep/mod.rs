use std::env;
use std::process;

mod lib;

use lib::Config;

pub fn run() {
    let args: Vec<_> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
