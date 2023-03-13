use anyhow::{Context, Ok, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::file_generate::FileGenerator;

/// A file sorter
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "trr")]
#[command(about = "A file sorter", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Generate(FileGenerator),
    Sort,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match self.command {
            Commands::Generate(ref filegen) => filegen.generate(),
            Commands::Sort => Ok(()),
        }
    }
}
