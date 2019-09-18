use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

const FILENAME: &str = "target/hello.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open(FILENAME)?;

    Ok(())
}

pub fn run() {
    let f = File::open(FILENAME);

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILENAME) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // Cleaner without a bunch of match statements
    let f = File::open(FILENAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILENAME).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for panic, `unwrap` and `expect`
    let f = File::open(FILENAME).unwrap();
    let f = File::open(FILENAME).expect("Failed to open hello.txt");

    // Propagating errors
    let username = read_username_from_file();

    // Shortcut propagating `?`
    // Operator only works in functions which return a Result<T, E>
    let username = read_username_from_file_shortcut();
    let username = read_username_from_file_shorter();
    let username = read_username_from_file_shortest();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open(FILENAME);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open(FILENAME)?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(FILENAME)?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string(FILENAME)
}
