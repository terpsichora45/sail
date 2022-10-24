#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io, env, fs, process::Command};

mod system;
mod parse;

use system::*;
use parse::*;

const HELP: &str = "
\t- Help Menu -
help       - displays the current menu.
init       - creates the directories and populates the files.
build      - compiles C files into Object files and compiles into an executable.
run        - if created, executes the binary.
new-header - creates a new header file within the project directory in the proper location.
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut command_arg: &str = ""; // TODO: update the command argument passing
    if args.len() == 1 {
        error!("Invalid ammount of arguments. Please use the format `sail <command> <arguments>`");
    }

    let command = args[1].as_str();
    if args.len() == 3 {
        command_arg = args[2].as_str();
    }

    match command {
        "help" => {
            println!("{HELP}");
        },
        "init" => {
            notify!("Creating directory {}...", command_arg);
            system::create_directory(command_arg);
        },
        "build" => {
            notify!("Compiling C project...");
            notify!("!!! MAKE SURE YOU HAVE `make` INSTALLED !!!");
            Command::new("make")
                .output()
                .expect("Failed to execute make command. Make sure you have `make` installed.");
        },
        "run" => {
            let mut directories: Vec<fs::DirEntry> = vec![];
            let mut files: Vec<fs::DirEntry> = vec![];
            let paths = fs::read_dir("./").unwrap();

            for path in paths {
                let p = path.unwrap();
                let file_type = p.file_type().unwrap();

                if file_type.is_dir() { directories.push(p); }
                else if !file_type.is_dir() { files.push(p); }

            }
            println!("{:?}", directories);
            println!("{:?}", files);
        },
        _ => {
            error!("Please provide a valid command.");
        }
    }
}
