use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::common::FrequencyBand;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStatistics {
    pub uptime_sec: i64,
    pub last_heartbeat_at: DateTime<Utc>,
    pub next_heartbeat_at: DateTime<Utc>,
    #[serde(default, rename = "loadAverage1Min")]
    pub load_average_1min: Option<f64>,
    #[serde(default, rename = "loadAverage5Min")]
    pub load_average_5min: Option<f64>,
    #[serde(default, rename = "loadAverage15Min")]
    pub load_average_15min: Option<f64>,
    pub cpu_utilization_pct: Option<f64>,
    pub memory_utilization_pct: Option<f64>,
    #[serde(default)]
    pub uplink: Option<DeviceUplinkStatistics>,
    #[serde(default)]
    pub interfaces: Option<DeviceInterfaceStatistics>,
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
    #[serde(default)]
    pub radios: Vec<WirelessRadioStatistics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WirelessRadioStatistics {
    #[serde(default, rename = "frequencyGHz")]
    pub frequency_ghz: Option<FrequencyBand>,
    #[serde(rename = "txRetriesPct")]
    pub tx_retries_pct: Option<f64>,
}