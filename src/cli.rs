use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    /// Override the search mechanism with a custom device IP
    #[clap(long)]
    pub override_ip: Option<String>,

    /// An SSH password override
    #[clap(long)]
    pub ssh_password: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Switches the device fans into automatic control
    Auto,
    /// Manually set the fan speed byte
    Set {
        /// The fan speed. Expressed as a value between 0 and 255.
        speed: u8,
    },
}
