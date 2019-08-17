mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute use
// use crate::ch07_04_module_use::front_of_house::hosting;

// Relative use
// use self::front_of_house::hosting;

// Re-export the `hosting` module
pub use crate::ch07_04_module_use::front_of_house::hosting;

// We wouldn't want to do the following, because it is unclear where the
// `add_to_wailist` function is defined
// use self::front_of_house::hosting::add_to_waitlist
//
// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// It's idiomatic to specify full path with structs/enums
use std::collections::HashMap;

// Don't specify full path for structs if they have same name
use std::fmt;
use std::io;

// Can use `as` to rename a use import
use std::fmt::Result;
use std::io::Result as IoResult;

// Nested paths
// use std::cmp::Ordering;
// use std::io;

// Could write the above ^^ as:
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// use std::io::{self, Write};

// Bring in all public items into scope
// use std::collections::*;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

fn function3() -> Result {
    Ok(())
}

fn function4() -> IoResult<()> {
    Ok(())
}

pub fn run() {
    let mut map = HashMap::new();

    map.insert(1, 2);
}
