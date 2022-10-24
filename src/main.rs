#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::{io, env, fs, process::*};

mod system;
mod parse;

use system::*;
use parse::*;

const HELP: &str = "
\t- Help Menu -
help       - displays the current menu
init       - creates the directories and populates the files
build      - compiles C files into Object files and compiles into an executable
run        - if created, executes the binary
new-header - creates a new header file within the project directory in the proper location
";

fn run_command(command: &str) -> u8 {
    working!("Running program");
    let program_output = match Command::new(command).output() {
        Ok(a) => a,
        Err(e) => { error!("{}", e); }
    };
    let output = String::from_utf8(program_output.stdout);
    let errput = String::from_utf8(program_output.stderr);

    complete!("Completed program Output: {:?}, Errput: {:?}", output, errput);
    0
}

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

    // TODO: Create a command system
    // ? Perhaps you can use stringify!() to some extent to adapt the function names to command titles
    match command {
        "help" => {
            println!("{HELP}");
        },
        "init" => {
            working!("Creating C project directory: `{}`", command_arg);
            system::create_directory(command_arg);
        },
        "build" => {
            working!("Compiling C project");
            swarn!("MAKE SURE YOU HAVE `make` INSTALLED");
            run_command("make ./test-project/");
            complete!("Executed make");
        },
        "run" => {
            let mut directories: Vec<fs::DirEntry> = vec![];
            let mut files: Vec<fs::DirEntry> = vec![];
            let paths = fs::read_dir("./test-project").unwrap();

            for path in paths {
                let p = path.unwrap();
                let file_type = p.file_type().unwrap();

                if file_type.is_dir() { directories.push(p); }
                else if !file_type.is_dir() { files.push(p); }

            }
            println!("Directories found: {:?}", directories);
            println!("Files found: {:?}", files);

            working!("Executing program");
            Command::new("./test-project/main.exe").output().expect("Failed to execute project binary.");
            complete!("Completed program execution");
        },
        _ => {
            error!("Please provide a valid command");
        }
    }
}
