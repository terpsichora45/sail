#![allow(unused_macros)]

use std::{fs, error::Error, process};

macro_rules! working {
    ($expression: expr) => {
        println!("{}", concat!("[\x1b[38;2;0;255;255m\x1b[1m .. \x1b[0m] ", $expression));
    };
    ($format: expr $(,$args: expr)*) => {
        println!(concat!("[\x1b[38;2;0;255;255m\x1b[1m .. \x1b[0m] ", $format) $(,$args)*);
    };
}

/// Prints a notification message
macro_rules! complete {
    ($expression: expr) => {
        println!("{}", concat!("[\x1b[38;2;0;255;0m\x1b[1m -- \x1b[0m] ", $expression));
    };
    ($format: expr $(,$args: expr)*) => {
        println!(concat!("[\x1b[38;2;0;255;0m\x1b[1m -- \x1b[0m] ", $format) $(,$args)*);
    };
}

// Prints a warning message
macro_rules! swarn {
    ($expression: expr) => {
        println!("{}", concat!("[\x1b[38;2;255;255;0m\x1b[1m ?? \x1b[0m] ", $expression));
    };
    ($format: expr $(,$args: expr)*) => {
        println!(concat!("[\x1b[38;2;255;255;0m\x1b[1m ?? \x1b[0m] ", $format) $(,$args)*);
    };
}

/// Displays an error message
macro_rules! error {
    ($expression: expr) => {
        println!("{}", concat!("[\x1b[38;2;255;0;0m\x1b[1m !! \x1b[0m] ", $expression));
        std::process::exit(-1);
    };
    ($format: expr $(,$args: expr)*) => {
        println!(concat!("[\x1b[38;2;255;0;0m\x1b[1m !! \x1b[0m] ", $format) $(,$args)*);
        std::process::exit(-1);
    };
    () => {
        false
    }
}

/// Developer notification
macro_rules! dev {
    ($expression: expr) => {
        if crate::DEBUG != false {
            println!("{}", concat!("[\x1b[38;2;128;0;255m\x1b[1m ** \x1b[0m] ", $expression));
        }
    };
    ($format: expr $(,$args: expr)*) => {
        if crate::DEBUG != false {
            println!(concat!("[\x1b[38;2;128;0;255m\x1b[1m ** \x1b[0m] ", $format) $(,$args)*);
        }
    };
    () => {
        if crate::DEBUG == false { std::process::quit(-2); }
    }
}

pub(crate) use working;
pub(crate) use complete;
pub(crate) use swarn;
pub(crate) use error;
pub(crate) use dev;

pub fn create_directory(proj_name: &str) {
    let parent_dir = format!("./{}", proj_name);

    if let Err(e) = fs::create_dir(&parent_dir) {
        // dev!("This is a test of the first developer notification.");
        swarn!("Entity already exists");
        error!("Exiting due `os_{:?}`", e.raw_os_error().unwrap());
    } else {
        // create child directories
        fs::create_dir(format!("{}/src", &parent_dir)).unwrap();
        fs::create_dir(format!("{}/docs", &parent_dir)).unwrap();
        fs::create_dir(format!("{}/include", &parent_dir)).unwrap();

        fs::write(format!("{}/src/main.c", &parent_dir), "#include <stdio.h>\n\nint main(void) {\n\tprintf(\"Hello, world!\\n\");\n\treturn 0;\n}\n").unwrap();
        fs::write(format!("{}/README.md", &parent_dir), "# README").unwrap();
        fs::write(format!("{}/makefile", &parent_dir), "main:\n\tgcc -o main src/main.c\n").unwrap();
    } // create parent directory

    complete!("Successfully created project directory");
}