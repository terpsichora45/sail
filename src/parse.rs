#![allow(dead_code)]

/// Checks if the provided option string exists within the provided arguments
/// * `args`: a vector containing the provided command-line arguments
/// * `opt`: the string that you wish to search for in argument form. (i.e. '-v' '--verbose')
fn exists(args: Vec<String>, opt: String) -> bool {
    args.contains(&opt)
}