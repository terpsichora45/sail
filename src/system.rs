#![allow(unused_macros)]

use std::fs;

/// Prints a notification message
macro_rules! notify {
    ($expression: expr) => {
        println!("{}", concat!("[\x1b[38;2;0;255;0m NOTIF \x1b[0m] ", $expression));
    };
    ($format: expr $(,$args: expr)*) => {
        println!(concat!("[\x1b[38;2;0;255;0m NOTIF \x1b[0m] ", $format) $(,$args)*);
    };
}

/// Displays an error message
macro_rules! error {
    ($expression: expr) => {
        println!("[\x1b[38;2;255;0;0m ERROR \x1b[0m] {}", $expression);
        std::process::exit(-1);
    };
}

pub(crate) use notify;
pub(crate) use error;

pub fn create_directory(proj_name: &str) {
    let parent_dir = format!("./{}", proj_name);

    match fs::create_dir(&parent_dir) {
        Ok(_) => {
            // create child directories
            fs::create_dir(format!("{}/src", &parent_dir)).unwrap();
            fs::create_dir(format!("{}/docs", &parent_dir)).unwrap();
            fs::create_dir(format!("{}/include", &parent_dir)).unwrap();

            fs::write(format!("{}/src/main.c", &parent_dir), "#include <stdio.h>\n\nint main(void) {\n\tprintf(\"Hello, world!\\n\");\n\treturn 0;\n}\n").unwrap();
            fs::write(format!("{}/README.md", &parent_dir), "# README").unwrap();
            fs::write(format!("{}/makefile", &parent_dir), "main:\n\tgcc -o main src/main.c\n").unwrap();
        },
        Err(e) => {
            error!(e.to_string());
        }
    } // create parent directory

    notify!("Successfully created directory!");
}