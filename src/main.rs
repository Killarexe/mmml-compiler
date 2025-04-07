mod args;
mod token;
mod lexer;
mod compiler;

use std::{fs::File, io::{Error, Write}, process::exit};

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

    let mut result: String = format!("const unsigned char {}[{}] = {{\n\t", args.get_music_name(), data.len());
    let bytes_per_line = 17;

    let formatted_bytes: Vec<String> = data.iter().enumerate()
        .map(|(idx, &byte)| {
            let byte_str = format!("0x{:02X}", byte);
            if (idx + 1) % bytes_per_line == 0 && idx < data.len() - 1 {
                format!("{},\n\t", byte_str)
            } else if idx < data.len() - 1 {
                format!("{},", byte_str)
            } else {
                byte_str
            }
        })
        .collect();

    result.push_str(&formatted_bytes.join(""));
    result.push_str("\n};");

    if args.verbose {
        println!("Result:\n{}", result);
    }

    let mut file: File = File::create(args.get_output_path())?;
    file.write_all(result.as_bytes())?;

    Ok(())
}
