#![allow(dead_code)]

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
    args.contains(&String::from(opt))
}

enum Actions {
    Help,
    Init,
    Build,
    Run,
    Test,
}
struct ActionFlags {
    debug: bool,
    verbose: bool,
    silent: bool,
    manualFileName: bool,
}

/// Iterates through the provided arguments and determines the programs functionality based on the provided information
/// * `args`: a vector containing the provided command-line arguments
#[deprecated = "not implemented yet"]
pub fn parse_arguments(args: Vec<String>) {
    /*
    Format:
    sail run test-project --verbose --debug

    Turns into:
    run(test-project, true, _, _, true);
    */
    // ! run() should take Actions and ActionFlags as well as the command argument provided to the run command. i.e.: run(arg, actions, actionFlags)
    
}