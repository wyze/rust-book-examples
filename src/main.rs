//! # Rust Book examples
//!
//! `rust-book-examples` is my attempt at writing examples from the book.

// Only allowing them as I am following the examples in the book
#![allow(
    clippy::eq_op,
    clippy::let_and_return,
    clippy::many_single_char_names,
    clippy::new_ret_no_self,
    clippy::ptr_arg,
    clippy::redundant_pattern_matching,
    clippy::single_match,
    clippy::trivially_copy_pass_by_ref,
    dead_code,
    unused_assignments,
    unused_mut,
    unused_variables
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
mod ch08_company_directory;
mod ch08_mean_median_mode;
mod ch08_pig_latin;
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
mod ch15_01_box;
mod ch15_02_deref;
mod ch15_03_drop;
mod ch15_04_rc;
mod ch15_05_interior_mutability;
mod ch15_06_ref_cycle;
mod ch16_01_threads;
mod ch16_02_message_passing;
mod ch16_03_shared_state;
mod ch17_01_oop;
mod ch17_02_trait_objects;
mod ch17_03_oo_patterns;
mod ch18_01_pattern_uses;
mod ch18_03_pattern_syntax;
mod ch19_01_unsafe_rust;
mod ch19_03_advanced_traits;
mod ch19_04_advanced_types;
mod ch19_05_advanced_functions_closures;
mod ch19_06_macros;
mod ch20_web_server;

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
use ch08_company_directory as company_directory;
use ch08_mean_median_mode as mean_median_mode;
use ch08_pig_latin as pig_latin;
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
use ch15_01_box as box_t;
use ch15_02_deref as deref;
use ch15_03_drop as drop;
use ch15_04_rc as rc;
use ch15_05_interior_mutability as interior_mutability;
use ch15_06_ref_cycle as ref_cycle;
use ch16_01_threads as threads;
use ch16_02_message_passing as message_passing;
use ch16_03_shared_state as shared_state;
use ch17_01_oop as oop;
use ch17_02_trait_objects as trait_objects;
use ch17_03_oo_patterns as oo_patterns;
use ch18_01_pattern_uses as pattern_uses;
use ch18_03_pattern_syntax as pattern_syntax;
use ch19_01_unsafe_rust as unsafe_rust;
use ch19_03_advanced_traits as advanced_traits;
use ch19_04_advanced_types as advanced_types;
use ch19_05_advanced_functions_closures as advanced_functions_closures;
use ch19_06_macros as macros;
use ch20_web_server as webserver;

use std::env;

enum Command {
    CompanyDirectory,
    Default,
    Game,
    MeanMedianMode,
    Minigrep,
    PigLatin,
    WebServer,
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
            box_t::run();
            deref::run();
            drop::run();
            rc::run();
            interior_mutability::run();
            ref_cycle::run();
            threads::run();
            message_passing::run();
            shared_state::run();
            oop::run();
            trait_objects::run();
            oo_patterns::run();
            pattern_uses::run();
            pattern_syntax::run();
            unsafe_rust::run();
            advanced_traits::run();
            advanced_types::run();
            advanced_functions_closures::run();
            macros::run();
        }
        Command::Game => guessing_game::run(),
        Command::MeanMedianMode => mean_median_mode::run(),
        Command::Minigrep => minigrep::run(),
        Command::PigLatin => pig_latin::run(),
        Command::WebServer => webserver::run(),
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
        "web-server" => Command::WebServer,
        _ => Command::Default,
    }
}
