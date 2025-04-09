use mmml_compiler::{args::{CompilerArgs, ExportType}, compiler::Compiler, lexer::Lexer, token::Token};
use std::{fs::File, io::{Error, Write}, path::PathBuf, process::exit};
use clap::Parser;

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
    let bytes: &[u8] = match args.export_type {
        ExportType::Code => {
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
            result.as_bytes()
        },
        ExportType::Raw => {
            data.as_slice()
        }
    };

    let mut file: File = File::create(args.get_output_path())?;
    file.write_all(bytes)?;

    if args.export_type == ExportType::Code {
        let mut path: PathBuf = args.get_output_path();
        path.set_extension("h");
        let mut header_file: File = File::create(path)?;
        let music_name: String = args.get_music_name();
        let header: String = format!("#ifndef {0}_H\n#define {0}_H\n\nextern const unsigned char {0}[];\n\n#endif", music_name);
        header_file.write_all(header.as_bytes())?;
    }

    Ok(())
}
