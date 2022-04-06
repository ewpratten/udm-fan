use clap::StructOpt;

mod cli;
mod discovery;

pub fn main() {
    // Get the CLI args
    let args = cli::Args::parse();
}
