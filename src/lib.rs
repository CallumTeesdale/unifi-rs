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
//! use unifi_rs::{client::{UnifiClient, UnifiClientBuilder}};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = UnifiClientBuilder::new("https://192.168.1.1/proxy/network/integrations")
//!         .api_key("your-api-key")
//!         .verify_ssl(false)
//!         .build()?;
//!
//!     let sites = client.list_sites(None, None).await?;
//!     println!("Sites: {:#?}", sites);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod errors;
pub mod models;

#[cfg(test)]
mod tests {
    use crate::client::{ErrorResponse, UnifiClientBuilder};
    use crate::models::client::ClientOverview;
    use crate::models::device::DeviceDetails;
    use crate::models::statistics::DeviceStatistics;
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
            ClientOverview::Wired(_) => {}
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

    #[tokio::test]
    async fn test_device_statistics_deserialization() {
        let stats_json = r#"{
        "uptimeSec": 737201,
        "lastHeartbeatAt": "2025-01-18T20:26:02Z",
        "nextHeartbeatAt": "2025-01-18T20:26:07Z",
        "loadAverage1Min": 1.65,
        "loadAverage5Min": 1.28,
        "loadAverage15Min": 1.3,
        "cpuUtilizationPct": 30.8,
        "memoryUtilizationPct": 74.2,
        "uplink": {
            "txRateBps": 309720,
            "rxRateBps": 32288
        },
        "interfaces": {
            "radios": [
                {
                    "frequencyGHz": 2.4,
                    "txRetriesPct": 14.3
                },
                {
                    "frequencyGHz": 5,
                    "txRetriesPct": 0
                }
            ]
        }
    }"#;

        let stats: DeviceStatistics = match serde_json::from_str(stats_json) {
            Ok(stats) => stats,
            Err(e) => {
                panic!("Failed to deserialize JSON: {}", e);
            }
        };

        assert_eq!(stats.uptime_sec, 737201, "uptime_sec does not match");
        assert_eq!(
            stats.cpu_utilization_pct,
            Some(30.8),
            "cpu_utilization_pct does not match"
        );
        assert!(
            stats.memory_utilization_pct.is_some(),
            "memory_utilization_pct is None"
        );
        assert!(stats.uplink.is_some(), "uplink is None");
        assert!(stats.interfaces.is_some(), "interfaces is None");

        let interfaces = stats.interfaces.as_ref().unwrap();
        assert!(!interfaces.radios.is_empty(), "radios is empty");
        assert_eq!(interfaces.radios.len(), 2, "radios length does not match");

        let radio_0 = &interfaces.radios[0];
        assert!(
            radio_0.frequency_ghz.is_some(),
            "radio_0 frequency_ghz is None"
        );

        let radio_1 = &interfaces.radios[1];
        assert!(
            radio_1.frequency_ghz.is_some(),
            "radio_1 frequency_ghz is None"
        );
    }
}
