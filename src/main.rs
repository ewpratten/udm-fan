use std::{net::Ipv4Addr, str::FromStr};

use clap::StructOpt;
use colored::Colorize;

mod cli;
mod discovery;
mod password;

#[tokio::main]
pub async fn main() {
    // Get the CLI args
    let args = cli::Args::parse();

    // Attempt to discover the UDM-PRO
    let udm_address = match args.override_ip {
        Some(ip) => Ipv4Addr::from_str(&ip).ok(),
        None => discovery::discover_udm_pro().await.unwrap(),
    };

    // Handle being unable to find a UDM-PRO
    if let None = udm_address {
        eprintln!(
            "{}\nTry using {} to specify an address manually",
            "Unable to find a UDM-Pro.".red(),
            "--override-ip".yellow()
        );
        std::process::exit(1);
    }
    let udm_address = udm_address.unwrap();

    // Print the UDM-PRO's IP address
    println!("Using {} to reach UDM-Pro", udm_address.to_string().cyan());

    // Get the SSH password
    let ssh_password = match args.ssh_password {
        Some(p) => p,
        None => password::get_udm_password(udm_address).to_string(),
    };
}
