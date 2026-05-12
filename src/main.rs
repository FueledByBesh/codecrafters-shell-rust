#[allow(unused_imports)]
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static BUILTIN_COMMANDS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["exit","type", "echo", "pwd"].into_iter().collect());

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();
        let command_vec: Vec<&str> = command.split_whitespace().collect();
        if command_vec[0] == "exit" {
            break;
        }
        if BUILTIN_COMMANDS.contains(&command_vec[0]) {
            exec_builtin_commands(&command_vec, command);
        } else {
            if let Some(_) = find_executable_from_path(command_vec[0]) {
                execute(&command_vec[0], &command_vec[1..])
            } else {
                print!("{}: command not found\n", command_vec[0]);
            }
        }
    }
}

fn exec_builtin_commands(command_vec: &[&str], command: &str) {
    if command_vec[0] == "type" {
        if command_vec.len() < 2 {
            println!("type: missing operand");
            return;
        } else if command_vec.len() > 2 {
            println!("type: too many arguments");
            return;
        }
        if BUILTIN_COMMANDS.contains(&command_vec[1]) {
            println!("{} is a shell builtin", command_vec[1]);
        } else {
            if let Some(x) = find_executable_from_path(&command_vec[1]) {
                println!("{} is {}", command_vec[1], x);
            } else {
                println!("{}: not found", command_vec[1])
            }
        }
    } else if command_vec[0] == "echo" {
        println!("{}", &command[5..]);
    } else if command_vec[0] == "pwd" {
        let current_dir: String = env::current_dir().ok()
            .and_then(|t| t.to_str().map(String::from))
            .unwrap_or("Couldn't find current dir".to_string());
        println!("{}", current_dir)
    } else {
        panic!("{}: unhandled builtin command!", command_vec[0])
    }
}

fn find_executable_from_path(command: &str) -> Option<String> {
    let path = env::var("PATH").unwrap();
    let paths: Vec<&str> = path.split(&[':', ';'][..]).collect();
    for path in paths {
        let full_path = Path::new(path).join(command);
        if full_path.exists() {
            //check for executable
            if is_executable(&full_path) {
                return full_path.to_str().map(String::from);
            }
        }
    }
    None
}

use std::fs;
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
fn is_executable(path: &PathBuf) -> bool {
    fs::metadata(path)
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
/**
trusts that the file is executable
*/
fn execute(command: &str, args: &[&str]) {
    use std::process::Command;
    let output = Command::new(command).args(args).output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout))
}
