use clap::Parser;
use trr::cli::Cli;
use anyhow::Result;

fn main() -> Result<()> {
    let arg = Cli::parse();
    arg.run()
}
