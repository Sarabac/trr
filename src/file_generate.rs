use anyhow::{Context, Ok, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use rand::prelude::*;
use std::{fs::create_dir_all, fs::File, path::PathBuf};

use crate::file_name_iter::FileNameIter;

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct FileGenerator {
    ///Path to the directory to fill
    directory: PathBuf,
    ///How many file to generate
    #[arg(short, long)]
    number: Option<usize>,
}

impl FileGenerator {
    pub fn generate(&self) -> Result<()> {
        let size = match self.number {
            Some(nb) => nb,
            None => 10,
        };
        let rng = rand::thread_rng();
        let sampler = FileNameIter::default();

        create_dir_all(self.directory.clone()).with_context(|| "can not create directory")?;
        sampler
            .sample_iter(rng)
            .take(size)
            .map(|filepath| self.directory.join(filepath))
            .map(|filename| File::create(filename))
            .fold(Ok(()), |acc, curr| {
                acc.and(curr.map(|_| ()).with_context(|| "error creating file"))
            })
    }
}
