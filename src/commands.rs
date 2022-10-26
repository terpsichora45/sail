#![allow(dead_code, unused_variables)]

use crate::{HELP, parse::ActionFlags, system::*};

pub fn help() {
    println!("{HELP}");
}

pub fn init(operand: String, flags: ActionFlags) {
    working!("Creating C project directory: `{}`", operand);
    create_directory(operand);
}

pub fn build(operand: String, flags: ActionFlags) {
    working!("Compiling C project");
    swarn!("MAKE SURE YOU HAVE `make` INSTALLED"); // replace this with some check for make installation

    change_dir(operand);
    run_command("make");

    complete!("Executed make");
    complete!("Completed compilation");
}


pub fn run(operand: String, flags: ActionFlags) {
    read_dir(operand.clone());
    change_dir(operand);
    run_command("./main.exe");
}

pub fn test(operand: String, flags: ActionFlags) {
    create_file(operand, "#ifndef TEST_H\n#define TEST_H\n\nvoid test(void);\n\n#endif");
}