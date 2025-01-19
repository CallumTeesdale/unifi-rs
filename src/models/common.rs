use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::Error;

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
pub struct ApplicationInfo {
    pub application_version: String,
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

#[derive(Debug, Clone, Serialize, PartialEq)]
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

impl<'de> Deserialize<'de> for FrequencyBand {
    fn deserialize<D>(deserializer: D) -> Result<FrequencyBand, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FrequencyBandVisitor;

        impl de::Visitor<'_> for FrequencyBandVisitor {
            type Value = FrequencyBand;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string or number representing frequency band")
            }

            fn visit_str<E>(self, value: &str) -> Result<FrequencyBand, E>
            where
                E: de::Error,
            {
                match value {
                    "2.4" => Ok(FrequencyBand::Band2_4GHz),
                    "5" => Ok(FrequencyBand::Band5GHz),
                    "6" => Ok(FrequencyBand::Band6GHz),
                    "60" => Ok(FrequencyBand::Band60GHz),
                    _ => Err(E::custom(format!("invalid frequency band: {}", value))),
                }
            }

            fn visit_f64<E>(self, value: f64) -> Result<FrequencyBand, E>
            where
                E: de::Error,
            {
                match value {
                    2.4 => Ok(FrequencyBand::Band2_4GHz),
                    5.0 => Ok(FrequencyBand::Band5GHz),
                    6.0 => Ok(FrequencyBand::Band6GHz),
                    60.0 => Ok(FrequencyBand::Band60GHz),
                    _ => Err(E::custom(format!("invalid frequency band: {}", value))),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<FrequencyBand, E>
            where
                E: de::Error,
            {
                match value {
                    5 => Ok(FrequencyBand::Band5GHz),
                    6 => Ok(FrequencyBand::Band6GHz),
                    60 => Ok(FrequencyBand::Band60GHz),
                    _ => Err(E::custom(format!("invalid frequency band: {}", value))),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<FrequencyBand, E>
            where
                E: de::Error,
            {
                match value {
                    5 => Ok(FrequencyBand::Band5GHz),
                    6 => Ok(FrequencyBand::Band6GHz),
                    60 => Ok(FrequencyBand::Band60GHz),
                    _ => Err(E::custom(format!("invalid frequency band: {}", value))),
                }
            }
        }

        deserializer.deserialize_any(FrequencyBandVisitor)
    }
}