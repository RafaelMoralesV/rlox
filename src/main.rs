mod evaluate;
mod expr;
mod parser;
mod primitives;
mod scan;

use std::fs;
use std::process::Termination;

use crate::parser::RecursiveDescentParser;
use clap::Parser as ClapParser;
use clap::Subcommand;
use evaluate::eval;
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

enum ProgramState {
    Success,
    LexerError,
    ParserError,
    RuntimeException,
}

impl Termination for ProgramState {
    fn report(self) -> std::process::ExitCode {
        use std::process::ExitCode;

        match self {
            ProgramState::Success => ExitCode::SUCCESS,
            ProgramState::LexerError | ProgramState::ParserError => ExitCode::from(65),
            ProgramState::RuntimeException => ExitCode::from(70),
        }
    }
}

fn main() -> ProgramState {
    let args = Args::parse();

    let mut status = ProgramState::Success;

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
                        status = ProgramState::LexerError;
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

            let mut parser = RecursiveDescentParser::new(
                Lexer::new(&file_contents).filter_map(Result::ok).collect(),
            );

            match parser.parse() {
                Ok(expr) => println!("{expr}"),
                Err(e) => {
                    status = ProgramState::ParserError;
                    eprintln!("{e}");
                }
            }
        }
        Commands::Evaluate { filename } => {
            let file_contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut parser = RecursiveDescentParser::new(
                Lexer::new(&file_contents).filter_map(Result::ok).collect(),
            );

            let expr = parser.parse();

            match eval(expr.unwrap()) {
                Ok(expr) => println!("{expr}"),
                Err(e) => {
                    status = ProgramState::RuntimeException;
                    eprintln!("{e}");
                }
            }
        }
    };

    status
}
