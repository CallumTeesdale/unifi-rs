//! UniFi Network API client library
//!
//! This library provides a Rust interface to the UniFi Network API, allowing you to
//! programmatically monitor and manage UniFi deployments.
//!
//! # Authentication
//!
//! The client requires an API key for authentication. You can obtain an API key through
//! the UniFi UI:
//! 1. Open your Site in UniFi Site Manager at unifi.ui.com
//! 2. Navigate to Control Plane -> Admins & Users
//! 3. Select your Admin
//! 4. Click Create API Key
//! 5. Add a name for your API Key
//! 6. Copy and securely store the key
//!
//! # Example
//!
//! ```rust,no_run
//! use unifi_rs::{UnifiClient, UnifiClientBuilder};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = UnifiClientBuilder::new("https://192.168.1.1/proxy/network/integrations")
//!         .api_key("your-api-key")
//!         .verify_ssl(false)
//!         .build()?;
//!     
//!     let sites = client.list_sites(None, None).await?;
//!     println!("Sites: {:#?}", sites);
//!     Ok(())
//! }
//! ```

use chrono::{DateTime, Utc};
use reqwest::{Client, ClientBuilder, header};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UnifiError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error: {status_code} - {message}")]
    Api {
        status_code: u16,
        message: String,
    },
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("Configuration error: {0}")]
    Config(String),
}

/// Builder for configuring and creating a UnifiClient instance
pub struct UnifiClientBuilder {
    base_url: String,
    api_key: Option<String>,
    verify_ssl: bool,
}

impl UnifiClientBuilder {
    /// Create a new builder instance with the base URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            verify_ssl: true,
        }
    }

    /// Set the API key for authentication
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Configure whether to verify SSL certificates
    pub fn verify_ssl(mut self, verify: bool) -> Self {
        self.verify_ssl = verify;
        self
    }

    /// Build the UnifiClient with the configured settings
    pub fn build(self) -> Result<UnifiClient, UnifiError> {
        let api_key = self.api_key.ok_or_else(|| {
            UnifiError::Config("API key is required".to_string())
        })?;

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "X-API-KEY",
            header::HeaderValue::from_str(&api_key)
                .map_err(|e| UnifiError::Config(e.to_string()))?,
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .danger_accept_invalid_certs(!self.verify_ssl)
            .build()?;

        Ok(UnifiClient {
            client,
            base_url: self.base_url,
        })
    }
}

/// Main client for interacting with the UniFi Network API
#[derive(Clone)]
pub struct UnifiClient {
    client: Client,
    base_url: String,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub offset: i32,
    pub limit: i32,
    pub count: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    pub data: Vec<T>,
}

/// Site overview information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteOverview {
    pub id: Uuid,
    pub name: String,
}

/// Device states
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceState {
    Online,
    Offline,
    PendingAdoption,
    Updating,
    GettingReady,
    Adopting,
    Deleting,
    ConnectionInterrupted,
    Isolated,
}

/// Device overview information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceOverview {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub mac_address: String,
    pub ip_address: String,
    pub state: DeviceState,
    pub features: Vec<String>,
    pub interfaces: Vec<String>,
}

/// Device statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatistics {
    pub uptime_sec: i64,
    pub last_heartbeat_at: DateTime<Utc>,
    pub next_heartbeat_at: DateTime<Utc>,
    pub load_average_1min: f64,
    pub load_average_5min: f64,
    pub load_average_15min: f64,
    pub cpu_utilization_pct: f64,
    pub memory_utilization_pct: f64,
}

impl UnifiClient {
    /// List all sites (paginated)
    pub async fn list_sites(&self, offset: Option<i32>, limit: Option<i32>) -> Result<Page<SiteOverview>, UnifiError> {
        let url = format!("{}/v1/sites", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[
                ("offset", offset.unwrap_or(0)),
                ("limit", limit.unwrap_or(25)),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ErrorResponse = response.json().await?;
            Err(UnifiError::Api {
                status_code: error.status_code,
                message: error.message,
            })
        }
    }

    /// List devices for a specific site (paginated)
    pub async fn list_devices(
        &self,
        site_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Page<DeviceOverview>, UnifiError> {
        let url = format!("{}/v1/sites/{}/devices", self.base_url, site_id);
        let response = self.client
            .get(&url)
            .query(&[
                ("offset", offset.unwrap_or(0)),
                ("limit", limit.unwrap_or(25)),
            ])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ErrorResponse = response.json().await?;
            Err(UnifiError::Api {
                status_code: error.status_code,
                message: error.message,
            })
        }
    }

    /// Get latest statistics for a specific device
    pub async fn get_device_statistics(&self, site_id: Uuid, device_id: Uuid) -> Result<DeviceStatistics, UnifiError> {
        let url = format!(
            "{}/v1/sites/{}/devices/{}/statistics/latest",
            self.base_url, site_id, device_id
        );
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let error: ErrorResponse = response.json().await?;
            Err(UnifiError::Api {
                status_code: error.status_code,
                message: error.message,
            })
        }
    }

    /// Restart a specific device
    pub async fn restart_device(&self, site_id: Uuid, device_id: Uuid) -> Result<(), UnifiError> {
        let url = format!(
            "{}/v1/sites/{}/devices/{}/actions",
            self.base_url, site_id, device_id
        );
        let response = self.client
            .post(&url)
            .json(&DeviceAction {
                action: "RESTART".to_string(),
            })
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error: ErrorResponse = response.json().await?;
            Err(UnifiError::Api {
                status_code: error.status_code,
                message: error.message,
            })
        }
    }
}

#[derive(Debug, Serialize)]
struct DeviceAction {
    action: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,
    message: String,
}

// Add tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_builder() {
        let client = UnifiClientBuilder::new("https://example.com")
            .api_key("test-key")
            .verify_ssl(false)
            .build();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_builder_missing_api_key() {
        let client = UnifiClientBuilder::new("https://example.com")
            .verify_ssl(false)
            .build();
        assert!(client.is_err());
    }
}
