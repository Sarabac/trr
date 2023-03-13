use anyhow::{Context, Ok, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use rand::prelude::*;
use std::{fs::create_dir_all, fs::File, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct FileSorter {
    ///Path to the directory to fill
    directory: PathBuf,
    ///How many file to generate
    #[arg(short, long)]
    number: Option<usize>,
}
