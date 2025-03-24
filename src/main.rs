mod args;
mod token;
mod lexer;

use std::{io::Error, process::exit};

use args::CompilerArgs;
use clap::Parser;
use lexer::Lexer;
use token::Token;

fn main() {
    let args: CompilerArgs = CompilerArgs::parse();
    if let Err(err) = compile(args) {
        println!("Error: {}", err);
        exit(1);
    } else {
        println!("Compiled sucessfuly!");
    }
}

fn compile(args: CompilerArgs) -> Result<(), Error> {
    let source_code: String = std::fs::read_to_string(args.input_path.clone())?;

    if args.verbose {
        println!("Source code:\n{}", source_code);
    }

    let mut lexer: Lexer = Lexer::new(source_code);
    let tokens: Vec<Token> = lexer.tokenize()?;

    if args.verbose {
        println!("Tokens:\n{:#?}", tokens);
    }
    Ok(())
}
