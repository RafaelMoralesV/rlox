mod eval;
mod expr;
mod parser;
mod scan;
mod token;

use std::fs;
use std::process::ExitCode;

use crate::parser::parser::Parser;
use clap::Parser as ClapParser;
use clap::Subcommand;
use eval::eval;
use scan::lexer::Lexer;

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

    /// Evaluates the contents of a file.
    Evaluate { filename: String },
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
                        eprintln!("{e}");
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

            match parser.parse() {
                Ok(expr) => println!("{expr}"),
                Err(e) => {
                    errors_found = true;
                    eprintln!("{e}");
                }
            }
        }
        Commands::Evaluate { filename } => {
            let file_contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut parser =
                Parser::new(Lexer::new(&file_contents).filter_map(Result::ok).collect());

            let expr = parser.parse();
            let expr = eval(expr.unwrap());

            println!("{}", expr.unwrap());
        }
    };

    if errors_found {
        ExitCode::from(65)
    } else {
        ExitCode::SUCCESS
    }
}
