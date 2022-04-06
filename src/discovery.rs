use std::net::{IpAddr, Ipv4Addr};

use dns_lookup::lookup_addr;
use pnet::datalink;
use regex::Regex;
use reqwest::Client;

#[derive(Debug, thiserror::Error)]
pub enum NetDiscoveryError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

/// Attempt to discover a UDM-Pro on one of the host's network interfaces.
///
/// ## Discovery Process
///
/// This works by checking all interfaces for gateways then making HTTPS requests 
/// at them and seeing if a UniFi OS control panel is hit.
pub async fn discover_udm_pro() -> Result<Option<Ipv4Addr>, NetDiscoveryError> {
    // We need to keep track of a regex pattern for HTML response validation
    let html_title_re = Regex::new(r"<title>UniFi OS</title>").unwrap();

    // We also need to build a custom HTTP client that allows self-signed certificates
    let http_client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // Find all network interfaces and their IP addresses on this machine
    for iface in datalink::interfaces() {
        for addr in iface.ips {
            // For now, I only know how to handle IPv4 discovery
            match addr {
                pnet::ipnetwork::IpNetwork::V4(addr) => {
                    // If there is a UDM here, it'll likely be the router (addr 1)
                    if let Some(possible_udm) = addr.nth(1) {
                        // Now, we can perform a DNS lookup against the device to determine its hostname
                        if let Ok(hostname) = lookup_addr(&IpAddr::V4(possible_udm)) {
                            // Ensure we are looking at a gateway
                            if hostname == "_gateway" {
                                // Now, we need to make an HTTP request against this device to verify it is running `Unifi OS`
                                let response = http_client
                                    .get(&format!("https://{}/", possible_udm))
                                    .send()
                                    .await?;
                                if response.status().is_success() {
                                    // Check the title of the response body
                                    let body = response.text().await?;
                                    if html_title_re.is_match(&body) {
                                        // We found a UDM-PRO!
                                        return Ok(Some(possible_udm));
                                    }
                                }
                            }
                        }
                    }
                }
                pnet::ipnetwork::IpNetwork::V6(_) => { /* Unused */ }
            }
        }
    }

    Ok(None)
}
