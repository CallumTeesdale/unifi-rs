# unifi-rs

[![Crates.io](https://img.shields.io/crates/v/unifi-rs)](https://crates.io/crates/unifi-rs)
[![Documentation](https://docs.rs/unifi-rs/badge.svg)](https://docs.rs/unifi-rs)
[![License](https://img.shields.io/crates/l/unifi-rs)](LICENSE)

A Rust client library for the UniFi Network API that enables programmatic monitoring and management of UniFi deployments.

## Features

- Asynchronous API using tokio
- Strong typing with serde for serialization/deserialization
- Comprehensive error handling
- Pagination support
- SSL verification configuration
- API key authentication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
unifi-rs = "0.1.0"
```

# Quick Start 
```rust
use unifi_rs::{UnifiClient, UnifiClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = UnifiClientBuilder::new("https://192.168.1.1/proxy/network/integrations")
        .api_key("your-api-key")
        .verify_ssl(false)
        .build()?;
    let sites = client.list_sites(None, None).await?;
    println!("Sites: {:#?}", sites);
    Ok(())
}
```
