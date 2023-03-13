use anyhow::Result;
use clap::Parser;
use trr::cli::Cli;

fn main() -> Result<()> {
    let arg = Cli::parse();
    arg.run()
}
