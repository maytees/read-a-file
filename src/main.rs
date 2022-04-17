use spine;
use spine::commands;

use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_commands(args);
}

fn parse_commands(args: Vec<String>) {
    match args.get(1) {
        Some(cmd) => match cmd.as_str() {
            "hello" => commands::cmd_hello(args),
            _ => {
                // is a file
                let file: String = match args.get(1) {
                    Some(x) => x,
                    None() => {}
                };
            }
        },
        None => {
            //Display help
        }
    }
}
