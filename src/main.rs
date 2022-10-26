#![allow(non_snake_case)]
// #![allow(unused_imports)]

use std::env;

mod system;
mod parse;
mod commands;

use crate::parse::*;

pub const DEBUG: bool = true;
const HELP: &str = "
\t- Help Menu -
help       - displays the current menu
init       - creates the directories and populates the files
build      - compiles C files into Object files and compiles into an executable
run        - if created, executes the binary
new-header - creates a new header file within the project directory in the proper location
";

fn main() {
    let args: Vec<String> = env::args().collect();

    check_length(args.clone());
    parse_arguments(args.clone());
}
