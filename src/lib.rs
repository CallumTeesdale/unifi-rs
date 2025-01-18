//! UniFi Network API client library
//!
//! This library provides a Rust interface to the UniFi Network API, allowing you to
//! monitor and manage UniFi deployments.
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

pub struct UnifiClientBuilder {
    base_url: String,
    api_key: Option<String>,
    verify_ssl: bool,
}

impl UnifiClientBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            verify_ssl: true,
        }
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn verify_ssl(mut self, verify: bool) -> Self {
        self.verify_ssl = verify;
        self
    }

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

// Main client
#[derive(Clone)]
pub struct UnifiClient {
    client: Client,
    base_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub offset: i32,
    pub limit: i32,
    pub count: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteOverview {
    pub id: Uuid,
    pub name: String,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    pub application_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicePhysicalInterfaces {
    pub ports: Vec<EthernetPortOverview>,
    pub radios: Vec<WirelessRadioOverview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthernetPortOverview {
    pub idx: i32,
    pub state: PortState,
    pub connector: ConnectorType,
    pub max_speed_mbps: i32,
    pub speed_mbps: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PortState {
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectorType {
    RJ45,
    SFP,
    SFPPLUS,
    SFP28,
    QSFP28,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WirelessRadioOverview {
    pub wlan_standard: WlanStandard,
    pub frequency_ghz: FrequencyBand,
    pub channel_width_mhz: i32,
    pub channel: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WlanStandard {
    #[serde(rename = "802.11a")]
    IEEE802_11A,
    #[serde(rename = "802.11b")]
    IEEE802_11B,
    #[serde(rename = "802.11g")]
    IEEE802_11G,
    #[serde(rename = "802.11n")]
    IEEE802_11N,
    #[serde(rename = "802.11ac")]
    IEEE802_11AC,
    #[serde(rename = "802.11ax")]
    IEEE802_11AX,
    #[serde(rename = "802.11be")]
    IEEE802_11BE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrequencyBand {
    #[serde(rename = "2.4")]
    Band2_4GHz,
    #[serde(rename = "5")]
    Band5GHz,
    #[serde(rename = "6")]
    Band6GHz,
    #[serde(rename = "60")]
    Band60GHz,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDetails {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub supported: bool,
    pub mac_address: String,
    pub ip_address: String,
    pub state: DeviceState,
    pub firmware_version: String,
    pub firmware_updatable: bool,
    pub adopted_at: DateTime<Utc>,
    pub provisioned_at: DateTime<Utc>,
    pub configuration_id: String,
    pub uplink: DeviceUplinkInterface,
    pub features: DeviceFeatures,
    pub interfaces: DevicePhysicalInterfaces,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUplinkInterface {
    pub device_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFeatures {
    pub switching: Option<SwitchFeatureOverview>,
    pub access_point: Option<AccessPointFeatureOverview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchFeatureOverview {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointFeatureOverview {}

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
    pub uplink: DeviceUplinkStatistics,
    pub interfaces: DeviceInterfaceStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUplinkStatistics {
    pub tx_rate_bps: i64,
    pub rx_rate_bps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInterfaceStatistics {
    pub radios: Vec<WirelessRadioStatistics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WirelessRadioStatistics {
    pub frequency_ghz: FrequencyBand,
    pub tx_retries_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ClientOverview {
    #[serde(rename = "WIRED")]
    Wired(WiredClientOverview),
    #[serde(rename = "WIRELESS")]
    Wireless(WirelessClientOverview),
    #[serde(rename = "VPN")]
    Vpn(VpnClientOverview),
    #[serde(rename = "TELEPORT")]
    Teleport(TeleportClientOverview),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseClientOverview {
    pub id: Uuid,
    pub name: String,
    pub connected_at: DateTime<Utc>,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiredClientOverview {
    #[serde(flatten)]
    pub base: BaseClientOverview,
    pub mac_address: String,
    pub uplink_device_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WirelessClientOverview {
    #[serde(flatten)]
    pub base: BaseClientOverview,
    pub mac_address: String,
    pub uplink_device_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VpnClientOverview {
    #[serde(flatten)]
    pub base: BaseClientOverview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeleportClientOverview {
    #[serde(flatten)]
    pub base: BaseClientOverview,
}


impl UnifiClient {
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

    pub async fn get_device_details(&self, site_id: Uuid, device_id: Uuid) -> Result<DeviceDetails, UnifiError> {
        let url = format!(
            "{}/v1/sites/{}/devices/{}",
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

    pub async fn get_info(&self) -> Result<ApplicationInfo, UnifiError> {
        let url = format!("{}/v1/info", self.base_url);
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

    pub async fn list_clients(
        &self,
        site_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Page<ClientOverview>, UnifiError> {
        let url = format!("{}/v1/sites/{}/clients", self.base_url, site_id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

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

    #[tokio::test]
    async fn test_client_types() {
        let wired_json = r#"{
            "type": "WIRED",
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": "Desktop PC",
            "connectedAt": "2025-01-18T12:00:00Z",
            "ipAddress": "192.168.1.100",
            "macAddress": "00:11:22:33:44:55",
            "uplinkDeviceId": "123e4567-e89b-12d3-a456-426614174001"
        }"#;

        let client: ClientOverview = serde_json::from_str(wired_json).unwrap();
        match client {
            ClientOverview::Wired(_) => assert!(true),
            _ => panic!("Expected Wired client"),
        }
    }

    #[tokio::test]
    async fn test_device_details_deserialization() {
        let details_json = r#"{
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "name": "Test Device",
            "model": "UHDIW",
            "supported": true,
            "macAddress": "00:11:22:33:44:55",
            "ipAddress": "192.168.1.1",
            "state": "ONLINE",
            "firmwareVersion": "6.6.55",
            "firmwareUpdatable": true,
            "adoptedAt": "2025-01-18T12:00:00Z",
            "provisionedAt": "2025-01-18T12:00:00Z",
            "configurationId": "test123",
            "uplink": {
                "deviceId": "123e4567-e89b-12d3-a456-426614174001"
            },
            "features": {},
            "interfaces": {
                "ports": [],
                "radios": []
            }
        }"#;

        let details: DeviceDetails = serde_json::from_str(details_json).unwrap();
        assert_eq!(details.name, "Test Device");
        assert_eq!(details.model, "UHDIW");
        assert_eq!(details.firmware_version, "6.6.55");
    }

    #[tokio::test]
    async fn test_error_response_deserialization() {
        let error_json = r#"{
            "statusCode": 401,
            "message": "Unauthorized access"
        }"#;

        let error: ErrorResponse = serde_json::from_str(error_json).unwrap();
        assert_eq!(error.status_code, 401);
        assert_eq!(error.message, "Unauthorized access");
    }
}