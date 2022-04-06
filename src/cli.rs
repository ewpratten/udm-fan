use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
    #[clap(long)]
    override_ip: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Switches the device fans into automatic control
    Auto,
    /// Manually set the fan speed byte
    Set {
        /// The fan speed. Expressed as a value between 0 and 255.
        speed: u8,
    },
}
