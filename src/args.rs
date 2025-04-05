use std::path::PathBuf;

use clap::Parser;

/// A Compiler to convert MMML files to C source data files.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CompilerArgs {
    /// Input source code.
    pub input_path: PathBuf,
    /// Output file
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    /// Music name in the output file
    #[arg(short, long)]
    music_name: Option<String>,
    /// Output more info (Debug purpuses only)
    #[clap(short, long, action)]
    pub verbose: bool
}

impl CompilerArgs {
    pub fn get_output_path(&self) -> PathBuf {
        self.output_path.clone().unwrap_or(self.input_path.with_extension("h"))
    }

    pub fn get_music_name(&self) -> String {
        if let Some(name) = &self.music_name {
            return name.clone();
        }
        if let Some(file_name) = self.get_output_path().to_str() {
            let file: Vec<&str> = file_name.split('.').collect();
            return file[0].to_string().to_uppercase();
        }
        String::from("Music")
    }
}
