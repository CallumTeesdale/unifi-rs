use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::common::{ConnectorType, FrequencyBand, PortState, WlanStandard};

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
pub struct DevicePhysicalInterfaces {
    #[serde(default)]
    pub ports: Vec<EthernetPortOverview>,
    #[serde(default)]
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
#[serde(rename_all = "camelCase")]
pub struct WirelessRadioOverview {
    pub wlan_standard: Option<WlanStandard>,
    #[serde(default, rename = "frequencyGHz")]
    pub frequency_ghz: Option<FrequencyBand>,
    #[serde(default, rename = "channelWidthMHz")]
    pub channel_width_mhz: Option<i32>,
    pub channel: Option<i32>,
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
    pub adopted_at: Option<DateTime<Utc>>,
    pub provisioned_at: Option<DateTime<Utc>>,
    pub configuration_id: String,
    #[serde(default)]
    pub uplink: Option<DeviceUplinkInterface>,
    #[serde(default)]
    pub features: Option<DeviceFeatures>,
    #[serde(default)]
    pub interfaces: Option<DevicePhysicalInterfaces>,
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