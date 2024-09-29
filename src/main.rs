mod expr;
mod lexer;
mod parser;
mod token;

use std::fs;
use std::process::ExitCode;

use crate::parser::parser::Parser;
use clap::Parser as ClapParser;
use clap::Subcommand;
use lexer::AnalisisError;

use crate::lexer::Lexer;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a list of the Tokens inside the provided Filename.
    Tokenize { filename: String },

    /// Parses the provided Filename into an AST.
    Parse { filename: String },
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut errors_found = false;

    match args.command {
        Commands::Tokenize { filename } => {
            let file_contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            // Uncomment this block to pass the first stage
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
        }
        Commands::Parse { filename } => {
            let file_contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut parser =
                Parser::new(Lexer::new(&file_contents).filter_map(Result::ok).collect());

            for expr in parser.parse().iter() {
                match expr {
                    Ok(expr) => println!("{expr}"),
                    Err(e) => {
                        errors_found = true;
                        eprintln!("{e}");
                    }
                }
            }
        }
    };

    if errors_found {
        ExitCode::from(65)
    } else {
        ExitCode::SUCCESS
    }
}
