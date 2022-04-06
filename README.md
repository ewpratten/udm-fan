# UniFi Dream Machine Pro: Fan control tool
[![Crates.io](https://img.shields.io/crates/v/udm-fan)](https://crates.io/crates/udm-fan) 
[![Docs.rs](https://docs.rs/udm-fan/badge.svg)](https://docs.rs/udm-fan) 
[![Build](https://github.com/Ewpratten/udm-fan/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/udm-fan/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/udm-fan/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/udm-fan/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/udm-fan/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/udm-fan/actions/workflows/audit.yml)

`udm-fan` is a utility for interacting with the fans on a UDM-Pro over the network. It works by sending PWM fan control commands over the UDM's SSH management interface.

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install udm-fan
```

## Usage

The following commands can be used to control the fans on a local UDM-Pro:

```sh
# Set the fans to half speed
udm-fan set 128

# Set the fans to full speed (with a custom IP address)
udm-fan set 255 --override-ip 172.16.11.5

# Bring the fans back to automatic control
udm-fan auto
```

Passwords can be provided through:

- The `--ssh-password` flag
- The `UDM_SSH_PASS` environment variable
- A `.netrc` entry with the hostname being the UDM's IP address
- The terminal at runtime
