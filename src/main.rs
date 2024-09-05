mod lexer;
mod token;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use lexer::AnalisisError;

use crate::lexer::Lexer;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ExitCode::SUCCESS;
    }

    let command = &args[1];
    let filename = &args[2];

    let mut errors_found = false;

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
                        Err(e) => {
                            errors_found = true;
                            match e {
                                AnalisisError::UnrecognizedCharacter(line, c) => {
                                    eprintln!("[line {line}] Error: Unexpected character: {c}")
                                }
                                AnalisisError::UnterminatedString(line) => {
                                    eprintln!("[line {line}] Error: Unterminated string.")
                                }
                            }
                        }
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ExitCode::FAILURE;
        }
    }

    if errors_found {
        return ExitCode::from(65);
    } else {
        return ExitCode::SUCCESS;
    }
}
