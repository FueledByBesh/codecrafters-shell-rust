#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::{Path, PathBuf};

fn main() {

    const BUILTIN_COMMANDS: [&str; 3] = ["type","echo","exit"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();
        let command_vec: Vec<&str> = command.split_whitespace().collect();
        if command_vec[0] == "exit" {
            break;
        } else if command_vec[0] == "type"{
            if command_vec.len() < 2 {
                println!("type: missing operand");
                continue;
            }else if command_vec.len() > 2 {
                println!("type: too many arguments");
                continue;
            }
            if BUILTIN_COMMANDS.contains(&command_vec[1]){
                println!("{} is a shell builtin", command_vec[1]);
            }else{
                if let Some(x) = find_executable_from_path(&command_vec[1]) {
                    println!("{} is {}",command_vec[1], x);
                } else {
                    println!("{}: invalid command", command_vec[1])
                }
            }
        } else if command_vec[0] == "echo"{
            println!("{}", &command[5..]);
        }else {
            print!("{}: command not found\n", command_vec[0]);
        }

    }
}

fn find_executable_from_path(command: &str) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(&[':',';'][..]).collect();
    for path in paths{
        let full_path = Path::new(path).join(command);
        if full_path.exists(){
            //check for executable
            if is_executable(&full_path) {return full_path.to_str().map(String::from)}
        }
    }
    None
}

use std::fs;
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
fn is_executable(path: &PathBuf)->bool{
    fs::metadata(path).map(|m| m.is_file() && m.permissions().mode() & 0o111 !=0).unwrap_or(false)
}