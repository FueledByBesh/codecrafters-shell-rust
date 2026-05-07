#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_trim = input.trim();
        if input_trim == "exit" {
            break;
        }
        let command: Vec<&str> = input_trim.split_whitespace().into_iter().collect();
        if command[0] == "echo"{
            println!("{}", &command[1..].join(" "));
        }else {
            print!("{}: command not found\n", input_trim);
        }

    }
}
