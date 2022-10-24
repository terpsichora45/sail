static mut COMMANDS: Vec<&str> = vec![];

fn echo(args: Vec<String>) {
    println!("{}", args.get(0).unwrap());
}

fn test_command(args: Vec<String>) {
    println!("Hello, world!");
}

macro_rules! new_command {
    ($func_name: ident) => {
        unsafe {
            COMMANDS.push(stringify!($func_name))
        }
    };
}

fn main() {
    new_command!(echo);
    new_command!(test_command);
    
    unsafe {
        println!("{:?}", COMMANDS);
    }
}