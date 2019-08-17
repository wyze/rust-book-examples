// Only allowing them as I am following the examples in the book
#![allow(
    clippy::let_and_return,
    clippy::many_single_char_names,
    clippy::ptr_arg,
    clippy::single_match,
    dead_code,
    unused_assignments,
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

use std::env;

enum Command {
    Default,
    Game,
}

fn main() {
    match get_command() {
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
        }
        Command::Game => {
            guessing_game::run();
        }
    }
}

fn get_command() -> Command {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Command::Default;
    }

    match args[1].as_str() {
        "game" => Command::Game,
        _ => Command::Default,
    }
}
