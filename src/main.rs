#[allow(unused_imports)]
use std::io::{self, Write};
fn main() {
    const SUPPORTED_COMMANDS: [&str; 3] = ["type","echo","exit"];
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
            if SUPPORTED_COMMANDS.contains(&command_vec[1]){
                println!("{} is a shell builtin", command_vec[1]);
            }else{
                println!("{}: not found", command_vec[1]);
            }
        } else if command_vec[0] == "echo"{
            println!("{}", &command[5..]);
        }else {
            print!("{}: command not found\n", command_vec[0]);
        }

    }
}
