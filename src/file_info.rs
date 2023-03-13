use clap::{Args, Parser, Subcommand, ValueEnum};
use rand::prelude::*;
use std::{
    error::Error,
    fs::{remove_file, rename},
    path::PathBuf,
};
use std::{fs::create_dir_all, fs::File};

#[derive(Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum ConflictStrategy {
    Null,
    Ignore,
    Replace,
    Remove,
}

#[derive(Clone, Debug, PartialEq, Eq, Args)]
pub struct FileInfo {
    pub source: PathBuf,
    pub target: PathBuf,
    pub conflict_strategy: ConflictStrategy,
}

impl FileInfo {
    pub fn is_on_conflict(&self) -> bool {
        self.source.exists()
    }

    pub fn apply(self) -> Result<(), Self> {
        if self.is_on_conflict() {
            match self.conflict_strategy {
                ConflictStrategy::Null => Err(self),
                ConflictStrategy::Ignore => Ok(()),
                ConflictStrategy::Remove => remove_file(self.source.clone()).map_err(|_err| self),
                ConflictStrategy::Replace => rename(self.source.clone(), self.target.clone()).map_err(|_err| self),
            }
        } else {
            rename(self.source.clone(), self.target.clone()).map_err(|_err| self)
        }
    }

}
