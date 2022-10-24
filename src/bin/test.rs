use std::process::Command;

fn main() {
    let program_output = Command::new("./test-project/main.exe").output().expect("Failed to run binary");
    let output = String::from_utf8(program_output.stdout);
    let errput = String::from_utf8(program_output.stderr);
    println!("{}: {}", errput.unwrap().trim(), output.unwrap().trim());
}