use std::{
    net::{Ipv4Addr, TcpStream},
    str::FromStr,
};

use clap::StructOpt;
use colored::Colorize;
use ssh2::Session;

mod cli;
mod discovery;
mod password;

#[tokio::main]
pub async fn main() {
    // Get the CLI args
    let args = cli::Args::parse();

    // Attempt to discover the UDM-Pro
    let udm_address = match args.override_ip {
        Some(ip) => Ipv4Addr::from_str(&ip).ok(),
        None => discovery::discover_udm_pro().await.unwrap(),
    };

    // Handle being unable to find a UDM-Pro
    if let None = udm_address {
        eprintln!(
            "{}\nTry using {} to specify an address manually",
            "Unable to find a UDM-Pro.".red(),
            "--override-ip".yellow()
        );
        std::process::exit(1);
    }
    let udm_address = udm_address.unwrap();

    // Print the UDM-Pro's IP address
    println!("Using {} to reach UDM-Pro", udm_address.to_string().cyan());

    // Get the SSH password
    let ssh_password = match args.ssh_password {
        Some(p) => p,
        None => password::get_udm_password(udm_address).to_string(),
    };

    // Open an SSH connection to the UDM-Pro
    let tcp = TcpStream::connect(format!("{}:{}", udm_address, 22)).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Authenticate
    sess.userauth_password("root", &ssh_password).unwrap();
    if !sess.authenticated(){
        eprintln!("{}", "SSH authentication failed.".red());
        std::process::exit(1);
    }

    // Open a communication channel
    let mut channel = sess.channel_session().unwrap();
    println!("Connected via SSH");

    // Handle our subcommand
    match args.command{
        cli::Commands::Auto => {
            // Restart the fan service
            channel.exec("/etc/init.d/S04ubnt-fan-speed start").unwrap();
            println!("{}", "Fan service restarted.".green());
        },
        cli::Commands::Set { speed } => {
            // Disable the fan service
            channel.exec("killall -9 S04ubnt-fan-speed ubnt-fan-speed").unwrap();
            println!("Fan service disabled");

            // Enable PWM control
            channel = sess.channel_session().unwrap();
            channel.exec("echo 1 > /sys/class/hwmon/hwmon0/device/pwm1_enable").unwrap();
            channel = sess.channel_session().unwrap();
            channel.exec("echo 1 > /sys/class/hwmon/hwmon0/device/pwm2_enable").unwrap();
            println!("PWM control enabled");

            // Set the fan speeds
            channel = sess.channel_session().unwrap();
            channel.exec(&format!("echo {} > /sys/class/hwmon/hwmon0/device/pwm1", speed)).unwrap();
            channel = sess.channel_session().unwrap();
            channel.exec(&format!("echo {} > /sys/class/hwmon/hwmon0/device/pwm2", speed)).unwrap();
            println!("Fan speeds set to {}", speed.to_string().green());
            println!("Use `udm-fan auto` to disable manual override");
        },
    }
}
