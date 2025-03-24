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
    /// Output more info (Debug purpuses only)
    #[clap(short, long, action)]
    pub verbose: bool
}

impl CompilerArgs {
    pub fn get_output_path(&self) -> PathBuf {
        self.output_path.clone().unwrap_or(self.input_path.with_extension("h"))
    }
}
