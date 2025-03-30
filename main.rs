use std::io::{self, Write};
use std::process;
use std::str::FromStr;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts = input.trim().split(" ");
        let collection = parts.collect::<Vec<&str>>();

        match collection[0] {
            "exit" => {
                let exit_code: u32 = FromStr::from_str(collection[1]).unwrap();
                let x: u32 = 0;
                if exit_code == x {
                    process::exit(0);
                }
            }
            "echo" => {
                println!("{}", collection[1..].join(" "));
            }
            _ => {
                println!("{}: command not found", collection[0]);
            }
        }
    }
}
