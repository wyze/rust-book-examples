//! # Rust Book examples
//!
//! `rust-book-examples` is my attempt at writing examples from the book.

// Only allowing them as I am following the examples in the book
#![allow(
    clippy::eq_op,
    clippy::let_and_return,
    clippy::many_single_char_names,
    clippy::ptr_arg,
    clippy::single_match,
    dead_code,
    unused_assignments,
    unused_mut,
    unused_variables,
)]

mod ch01_03_hello_world;
mod ch02_00_guessing_game;
mod ch03_01_variables;
mod ch03_02_data_types;
mod ch03_03_functions;
mod ch03_05_control_flow;
mod ch04_01_ownership;
mod ch04_02_references;
mod ch04_03_slices;
mod ch05_01_structs_intro;
mod ch05_02_structs_example;
mod ch05_03_method_syntax;
mod ch06_01_enums;
mod ch06_02_match;
mod ch06_03_if_let;
mod ch07_02_module_scope;
mod ch07_03_module_paths;
mod ch07_04_module_use;
mod ch07_05_module_files;
mod ch08_01_vectors;
mod ch08_02_strings;
mod ch08_03_hash_maps;
mod ch08_mean_median_mode;
mod ch08_pig_latin;
mod ch08_company_directory;
mod ch09_02_result;
mod ch10_00_generics_intro;
mod ch10_01_generics_syntax;
mod ch10_02_traits;
mod ch10_03_lifetimes;
mod ch11_01_writing_tests;
mod ch12_minigrep;
mod ch13_01_closures;
mod ch13_02_iterators;
mod ch13_03_improving_io;
mod ch14_02_publishing;

use ch01_03_hello_world as hello_world;
use ch02_00_guessing_game as guessing_game;
use ch03_01_variables as variables;
use ch03_02_data_types as data_types;
use ch03_03_functions as functions;
use ch03_05_control_flow as control_flow;
use ch04_01_ownership as ownership;
use ch04_02_references as references;
use ch04_03_slices as slices;
use ch05_01_structs_intro as structs_intro;
use ch05_02_structs_example as structs_example;
use ch05_03_method_syntax as method_syntax;
use ch06_01_enums as enums;
use ch06_02_match as match_op;
use ch06_03_if_let as if_let;
use ch07_02_module_scope as module_scope;
use ch07_03_module_paths as module_paths;
use ch07_04_module_use as module_use;
use ch07_05_module_files as module_files;
use ch08_01_vectors as vectors;
use ch08_02_strings as strings;
use ch08_03_hash_maps as hash_maps;
use ch08_mean_median_mode as mean_median_mode;
use ch08_pig_latin as pig_latin;
use ch08_company_directory as company_directory;
use ch09_02_result as result;
use ch10_00_generics_intro as generics_intro;
use ch10_01_generics_syntax as generics_syntax;
use ch10_02_traits as traits;
use ch10_03_lifetimes as lifetimes;
use ch11_01_writing_tests as writing_tests;
use ch12_minigrep as minigrep;
use ch13_01_closures as closures;
use ch13_02_iterators as iterators;
use ch13_03_improving_io as improving_io;
use ch14_02_publishing as publishing;

use std::env;

enum Command {
    CompanyDirectory,
    Default,
    Game,
    MeanMedianMode,
    Minigrep,
    PigLatin,
}

fn main() {
    match get_command() {
        Command::CompanyDirectory => company_directory::run(),
        Command::Default => {
            hello_world::run();
            variables::run();
            data_types::run();
            functions::run();
            control_flow::run();
            ownership::run();
            references::run();
            slices::run();
            structs_intro::run();
            structs_example::run();
            method_syntax::run();
            enums::run();
            match_op::run();
            if_let::run();
            module_scope::run();
            module_paths::run();
            module_use::run();
            module_files::run();
            vectors::run();
            strings::run();
            hash_maps::run();
            result::run();
            generics_intro::run();
            generics_syntax::run();
            traits::run();
            lifetimes::run();
            writing_tests::run();
            closures::run();
            iterators::run();
            improving_io::run();
            publishing::run();
        },
        Command::Game => guessing_game::run(),
        Command::MeanMedianMode => mean_median_mode::run(),
        Command::Minigrep => minigrep::run(),
        Command::PigLatin => pig_latin::run(),
    }
}

fn get_command() -> Command {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Command::Default;
    }

    match args[1].as_str() {
        "company-directory" => Command::CompanyDirectory,
        "game" => Command::Game,
        "mean-median-mode" => Command::MeanMedianMode,
        "minigrep" => Command::Minigrep,
        "pig-latin" => Command::PigLatin,
        _ => Command::Default,
    }
}
