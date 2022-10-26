#![allow(dead_code)]

use std::ops::Deref;

use crate::system::*;

/// Validates the length of the provided vector of Strings
/// * `args`: a vector containing the provided command-line arguments
pub fn check_length(args: Vec<String>) {
    if args.len() == 1 {
        error!("Invalid ammount of arguments. Please use the format `sail <command> <arguments>`");
    }
}

/// Checks if the provided option string exists within the provided arguments
/// * `args`: a vector containing the provided command-line arguments
/// * `opt`: the string that you wish to search for in argument form. (i.e. '-v' '--verbose')
fn exists(args: Vec<String>, opt: &str, opt_expanded: &str) -> bool {
    args.contains(&String::from(opt)) || args.contains(&String::from(opt_expanded))
}

fn find_operand(args: Vec<String>) -> String {
    for arg in args.iter() {
        if !arg.starts_with('-') && *arg != args[0] && *arg != args[1] {
            return arg.deref().to_string();
        }
    }
    return String::from("");
}

pub enum Actions {
    Help,
    Init,
    Build,
    Run,
    Test,
}
pub struct ActionFlags {
    debug: bool,
    verbose: bool,
    silent: bool,
    manualFileName: bool,
}

/// Iterates through the provided arguments and determines the programs functionality based on the provided information
/// * `args`: a vector containing the provided command-line arguments
pub fn parse_arguments(args: Vec<String>) {
    /*
    Format:
    sail run test-project --verbose --debug

    Turns into:
    run(test-project, true, _, _, true);
    */
    // ! run() should take Actions and ActionFlags as well as the command argument provided to the run command. i.e.: run(arg, actions, actionFlags)
    
    // define the normal command
    check_length(args.clone());
    let operand = find_operand(args.clone());

    // check flags
    let flags: ActionFlags = ActionFlags {
        debug: exists(args.clone(), "-d", "--debug"),
        verbose: exists(args.clone(), "-v", "--verbose"),
        silent: exists(args.clone(), "-s", "--silent"),
        manualFileName: exists(args.clone(), "-o", "--output"),
    };
    let help_flag = exists(args.clone(), "-h", "--help");

    let command_str = args[1].as_str();
    let mut command: Actions = match command_str {
        "help" => Actions::Help,
        "init" => Actions::Init,
        "build" => Actions::Build,
        "run" => Actions::Run,
        _ => { Actions::Test }
    };
    if help_flag {
        command = Actions::Help;
    }

    execute_command(command, operand, flags);
}