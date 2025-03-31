use std::io::{self, Write};
use std::process::{self, Command};
use std::str::FromStr;
use std::env; 
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts = input.trim().split(" ");
        let collection = parts.collect::<Vec<&str>>();

        let built_in = vec!["echo","exit", "type"];

        match collection[0] {
            "exit" => {
                let exit_code: u32 = FromStr::from_str(collection[1]).unwrap();
                let x: u32 = 0;
                if exit_code == x {
                    process::exit(0);
                }
            }
            "type" => {
                let path = env::var("PATH").unwrap();
                let paths = path.split(":");
                let path_collection = paths.collect::<Vec<&str>>();

                if built_in.contains(&collection[1]) {
                    println!("{} is a shell builtin", collection[1]);
                } else {
                    let mut found = false;
                    for dir in &path_collection {
                        let cmd_path = Path::new(dir).join(collection[1]);
                        if let Ok(metadata) = fs::metadata(&cmd_path) {
                            if metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0 {
                                println!("{} is {}", collection[1], cmd_path.display());
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        println!("{}: not found", collection[1]);
                    }
                }
            }
            "echo" => {
                println!("{}", collection[1..].join(" "));
            }
            _ => {
                let status = Command::new(collection[0])
                    .args(&collection[1..])
                    .status();

                if let Err(_) = status {
                    println!("{}: command not found", collection[0]);
                }
            }
        }
    }
}
