mod front_of_house;

pub use crate::ch07_05_module_files::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn run() {}
