use spine::commands;

use std::env;
use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut source = String::new();

    if let Err(e) = parse_commands(args, &mut source) {
        println!("{}", e)
    }

    println!("{}", &source);
}

fn parse_commands(args: Vec<String>, source: &mut String) -> Result<(), std::io::Error> {
    match args.get(1) {
        Some(cmd) => match cmd.as_str() {
            "hello" => commands::cmd_hello(args),
            _ => {
                // Assume that the user passed a dir
                let file_path: String = match args.get(1) {
                    Some(x) => x.clone(),
                    None => "nothing".to_string(),
                };

                OpenOptions::new()
                    .read(true)
                    .open(file_path)?
                    .read_to_string(source)?;
            }
        },
        None => {
            //Display help
        }
    }

    Ok(())
}
