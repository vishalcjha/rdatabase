use std::{
    io::{self, Write},
    str::FromStr,
};

use front_end::CommandType;

fn main() {
    println!("Starting database");
    let mut cmd = String::new();
    print_prompt();
    while let Ok(_) = io::stdin().read_line(&mut cmd) {
        match CommandType::from_str(cmd.trim()) {
            Ok(command) => println!("received command {:?}", command),
            Err(err) => eprintln!("{:?}", err),
        }
        cmd.clear();
        print_prompt();
    }
}

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}
