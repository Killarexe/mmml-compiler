mod args;
mod token;
mod lexer;
mod compiler;

use std::{io::Error, process::exit};

use args::CompilerArgs;
use clap::Parser;
use compiler::Compiler;
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

    let mut compiler: Compiler = Compiler::new(tokens);
    let data: Vec<u8> = compiler.compile()?;

    let array_name: String = args.get_output_path().file_name().unwrap().to_str().unwrap().to_uppercase();
    let mut result: String = format!("unsigned char {}[{}] = {{", array_name, data.len());
    result.push_str(&data.iter().map(|&byte| format!("0x{:02X}", byte)).collect::<Vec<String>>().join(", "));
    result.push_str("};");
    println!("Result:\n{}", result);

    Ok(())
}
