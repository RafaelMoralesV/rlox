mod lexer;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};

use lexer::AnalisisError;

use crate::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                for token in Lexer::new(&file_contents) {
                    match token {
                        Ok(token) => println!("{token}"),
                        Err(e) => match e {
                            AnalisisError::UnrecognizedCharacter(c) => {
                                eprintln!("[line 1] Error: Unexpected Character {c}")
                            }
                        },
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
