use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub name: Option<String>,
    pub connected_at: DateTime<Utc>,
    #[serde(default)]
    pub ip_address: Option<String>,
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
