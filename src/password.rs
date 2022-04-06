use colored::Colorize;
use netrc_rs::Netrc;
use std::net::Ipv4Addr;

/// Automatically determines the password to use for SSH control
///
/// ## Steps
///
/// 1. CLI password
/// 2. Environment variable
/// 3. NetRC entry
/// 4. Prompt for password
pub fn get_udm_password(ip_addr: Ipv4Addr) -> String {
    // Handle CLI and ENV searching
    if let Some(password) = std::env::var("UDM_SSH_PASS").ok() {
        return password;
    };

    // Attempt to load a netrc file
    let netrc_path = shellexpand::tilde("~/.netrc");
    if let Ok(contents) = std::fs::read_to_string(netrc_path.to_string()) {
        if let Ok(netrc) = Netrc::parse(contents, false) {
            // Search the machine list for an entry containing the UDM-Pro's IP address
            for entry in netrc.machines {
                if entry.name == Some(ip_addr.to_string()) {
                    // Found an entry, return the password
                    if let Some(password) = entry.password {
                        return password;
                    }
                }
            }
        }
    }

    // If we managed to get here, we need to prompt for the password (and explain why)
    println!("Tried to automatically determine password, but failed.");
    println!("To automate this next time, try one of:\n  - Using the {} argument\n  - Setting the {} environment variable\n  - Adding an entry for the host {} to your .netrc", "--ssh-password".yellow(), "UDM_SSH_PASS".yellow(), ip_addr.to_string().cyan());

    // Prompt for the password
    return rpassword::prompt_password(format!("Password for {}: ", ip_addr.to_string().cyan()))
        .unwrap();
}
