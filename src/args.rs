use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq, Eq)]
pub enum ExportType {
    /// C code
    #[default]
    Code,
    /// Raw binary
    Raw
}

/// A Compiler to convert MMML files to C source data files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CompilerArgs {
    /// Input source code.
    pub input_path: PathBuf,
    /// Output file
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    /// Export type (C code or raw binary data)
    #[arg(short, long)]
    pub export_type: ExportType,
    /// Music name in the output file
    #[arg(short, long)]
    music_name: Option<String>,
    /// Output more info (Debug purpuses only)
    #[clap(short, long, action)]
    pub verbose: bool
}

impl CompilerArgs {
    pub fn get_output_path(&self) -> PathBuf {
        self.output_path.clone().unwrap_or(self.input_path.with_extension(
            match self.export_type {
                ExportType::Code => "c",
                ExportType::Raw => "mbf" // µMML Binary File
            }
        ))
    }

    pub fn get_music_name(&self) -> String {
        if let Some(name) = &self.music_name {
            return name.clone();
        }
        if let Some(file_name) = self.get_output_path().to_str() {
            let file: Vec<&str> = file_name.split('.').collect();
            return file[0]
                .to_string()
                .to_uppercase()
                .replace(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'], "");
        }
        String::from("Music")
    }
}
