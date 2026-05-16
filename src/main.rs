mod tests;

#[allow(unused_imports)]
use std::collections::HashSet;
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static BUILTIN_COMMANDS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| ["exit", "type", "echo", "pwd", "cd"].into_iter().collect());

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim();
        if command.is_empty() {
            continue;
        }
        let command_vec: Vec<String>;
        match tokenize(command) {
            Ok(v) => command_vec = v,
            Err(e) => {
                print!("{}", e);
                continue;
            }
        }
        if command_vec[0] == "exit" {
            break;
        }
        if BUILTIN_COMMANDS.contains(command_vec[0].as_str()) {
            exec_builtin_commands(&command_vec);
        } else {
            if let Some(_) = find_executable_from_path(command_vec[0].as_str()) {
                execute(&command_vec[0], &command_vec[1..])
            } else {
                print!("{}: command not found\n", command_vec[0]);
            }
        }
    }
}

fn exec_builtin_commands(command_vec: &[String]) {
    match command_vec[0].as_str() {
        "type" => {
            if command_vec.len() < 2 {
                println!("type: missing operand");
                return;
            } else if command_vec.len() > 2 {
                println!("type: too many arguments");
                return;
            }
            if BUILTIN_COMMANDS.contains(command_vec[1].as_str()) {
                println!("{} is a shell builtin", command_vec[1]);
            } else {
                if let Some(x) = find_executable_from_path(command_vec[1].as_str()) {
                    println!("{} is {}", command_vec[1], x);
                } else {
                    println!("{}: not found", command_vec[1])
                }
            }
        }
        "echo" => {
            println!("{}", command_vec[1..].join(" "));
        }
        "pwd" => {
            let current_dir: String = env::current_dir()
                .ok()
                .and_then(|t| t.to_str().map(String::from))
                .unwrap_or("Couldn't find current dir".to_string());
            println!("{}", current_dir)
        }
        "cd" => match command_vec.len() {
            1 => env::set_current_dir(env::home_dir().expect("Couldn't find home directory"))
                .expect("Couldn't change current directory"),
            2 => {
                let path = command_vec[1]
                    .starts_with("~")
                    .then(|| {
                        let home_dir = env::home_dir().expect("Couldn't get home directory!");
                        if command_vec[1].len() == 1 {
                            home_dir
                        } else {
                            if command_vec[1].starts_with("~/") {
                                home_dir.join(&command_vec[1][2..])
                            } else {
                                Path::new(&command_vec[1]).to_path_buf()
                            }
                        }
                    })
                    .unwrap_or(Path::new(&command_vec[1]).to_path_buf());
                if is_absolute_path_buf(&path) {
                    if path.exists() {
                        fs::metadata(&path)
                            .expect("Couldn't read metadata")
                            .is_dir()
                            .then(|| {
                                env::set_current_dir(&path)
                                    .expect("Couldn't change current directory")
                            })
                            .unwrap_or_else(|| {
                                println!("cd: {}: not a directory", path.to_str().unwrap())
                            })
                    } else {
                        println!("cd: {}: No such file or directory", path.to_str().unwrap())
                    }
                } else {
                    let cwd = env::current_dir().expect("Couldn't read current directory!");
                    env::set_current_dir(cwd.join(&command_vec[1])).unwrap_or_else(|e| {
                        match e.kind().to_string().as_str() {
                            "entity not found" => {
                                println!("cd: {}: No such file or directory", command_vec[1])
                            }
                            "not a directory" => {
                                println!("cd: {}: not a directory", command_vec[1])
                            }
                            _ => {
                                panic!("Couldn't change working directory, e:{e}")
                            }
                        }
                    })
                }
            }
            _ => println!("cd: too many arguments"),
        },
        _ => panic!("{}: unhandled builtin command!", command_vec[0]),
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
fn execute(command: &str, args: &[String]) {
    use std::process::Command;
    let output = Command::new(command).args(args).output().unwrap();
    print!("{}", String::from_utf8_lossy(&output.stdout))
}

// ==================== helper functions ================
// fn is_absolute_path(path: &str) -> bool {
//     path.starts_with("/")
// }
fn is_absolute_path_buf(path: &PathBuf) -> bool {
    path.is_absolute()
}

fn tokenize(command: &str) -> Result<Vec<String>, &'static str> {
    let mut tokens: Vec<String> = Vec::new();
    let mut single_quote_opened: bool = false;
    let mut double_quote_opened: bool = false;
    let mut backslash_opened: bool = false;
    let mut buffer: String = String::with_capacity(command.len());
    // let mut prev_char: Option<char> = None; //here whitespace doesn't count as character
    for c in command.chars() {
        if backslash_opened{
            buffer.push(c);
            backslash_opened = false;
            continue
        }
        match c {
            '"' => {
                if single_quote_opened {
                    buffer.push(c);
                } else {
                    double_quote_opened = !double_quote_opened;
                }
            }
            '\'' => {
                if double_quote_opened {
                    buffer.push(c);
                } else {
                    single_quote_opened = !single_quote_opened;
                }
            }
            '\\' => {
                if single_quote_opened || double_quote_opened {
                    buffer.push(c);
                } else {
                    backslash_opened = !backslash_opened;
                }
            }
            _ => {
                if single_quote_opened || double_quote_opened {
                    buffer.push(c);
                } else {
                    if c == ' ' {
                        if !buffer.is_empty() {
                            tokens.push(buffer.as_str().to_owned());
                            buffer.clear();
                        }
                    } else {
                        buffer.push(c);
                    }
                }
            }
        }
    }

    if single_quote_opened {
        return Err("invalid syntax! close single quotes!\n");
    }
    if double_quote_opened {
        return Err("invalid syntax! close double quotes!\n");
    }
    if !buffer.is_empty() {
        tokens.push(buffer.as_str().to_owned())
    }

    // print_tokenized(&tokens);

    Ok(tokens)
}

fn print_tokenized(vec: &Vec<String>) {
    vec.iter().for_each(|x| println!("{x}"))
}
