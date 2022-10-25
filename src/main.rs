#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io, env, fs, process::*, path::Path};

mod system;
mod parse;
mod commands;

use crate::system::*;
use crate::parse::*;
use crate::commands::*;

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
    let command = args[1].as_str();
    let mut command_arg: &str = ""; // TODO: update the command argument passing

    check_length(args.clone());
    parse_arguments(args.clone());

    if args.len() == 3 {
        command_arg = args[2].as_str();
    }

    match command {
        "help" => { println!("{HELP}"); },
        "init" => {
            working!("Creating C project directory: `{}`", command_arg);
            create_directory(command_arg);
        },
        "build" => {
            working!("Compiling C project");
            swarn!("MAKE SURE YOU HAVE `make` INSTALLED"); // replace this with some check for make installation

            change_dir(command_arg);
            run_command("make");

            complete!("Executed make");
            complete!("Completed compilation");
        },
        "run" => {
            read_dir(command_arg);
            change_dir(command_arg);
            run_command("./main.exe");
        },
        "test" => {
            create_file(command_arg, "#ifndef TEST_H\n#define TEST_H\n\nvoid test(void);\n\n#endif");
        }
        _ => { error!("Please provide a valid command"); }
    }
}
