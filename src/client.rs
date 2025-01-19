use crate::errors::UnifiError;
use crate::models::client::ClientOverview;
use crate::models::common::{ApplicationInfo, Page};
use crate::models::device::{DeviceDetails, DeviceOverview};
use crate::models::site::SiteOverview;
use crate::models::statistics::DeviceStatistics;
use reqwest::{header, Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        let api_key = self
            .api_key
            .ok_or_else(|| UnifiError::Config("API key is required".to_string()))?;

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

#[derive(Clone)]
pub struct UnifiClient {
    client: Client,
    base_url: String,
}

impl UnifiClient {
    /// Lists the sites available in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `offset` - An optional parameter to specify the starting point of the list.
    /// * `limit` - An optional parameter to specify the maximum number of sites to return.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Page` of `SiteOverview` on success, or a `UnifiError` on failure.
    pub async fn list_sites(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Page<SiteOverview>, UnifiError> {
        let url = format!("{}/v1/sites", self.base_url);
        let response = self
            .client
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

    /// Lists the devices available in the specified site in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The UUID of the site for which to list devices.
    /// * `offset` - An optional parameter to specify the starting point of the list.
    /// * `limit` - An optional parameter to specify the maximum number of devices to return.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Page` of `DeviceOverview` on success, or a `UnifiError` on failure.
    pub async fn list_devices(
        &self,
        site_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Page<DeviceOverview>, UnifiError> {
        let url = format!("{}/v1/sites/{}/devices", self.base_url, site_id);
        let response = self
            .client
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

    /// Retrieves the details of a specific device in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The UUID of the site containing the device.
    /// * `device_id` - The UUID of the device to retrieve details for.
    ///
    /// # Returns
    ///
    /// A `Result` containing `DeviceDetails` on success, or a `UnifiError` on failure.
    pub async fn get_device_details(
        &self,
        site_id: Uuid,
        device_id: Uuid,
    ) -> Result<DeviceDetails, UnifiError> {
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

    /// Retrieves the latest statistics for a specific device in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The UUID of the site containing the device.
    /// * `device_id` - The UUID of the device to retrieve statistics for.
    ///
    /// # Returns
    ///
    /// A `Result` containing `DeviceStatistics` on success, or a `UnifiError` on failure.
    pub async fn get_device_statistics(
        &self,
        site_id: Uuid,
        device_id: Uuid,
    ) -> Result<DeviceStatistics, UnifiError> {
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

    /// Restarts a specific device in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The UUID of the site containing the device.
    /// * `device_id` - The UUID of the device to restart.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or containing a `UnifiError` on failure.
    pub async fn restart_device(&self, site_id: Uuid, device_id: Uuid) -> Result<(), UnifiError> {
        let url = format!(
            "{}/v1/sites/{}/devices/{}/actions",
            self.base_url, site_id, device_id
        );
        let response = self
            .client
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

    /// Retrieves application information from the UniFi Network API.
    ///
    /// # Returns
    ///
    /// A `Result` containing `ApplicationInfo` on success, or a `UnifiError` on failure.
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

    /// Lists the clients available in the specified site in the UniFi Network API.
    ///
    /// # Arguments
    ///
    /// * `site_id` - The UUID of the site for which to list clients.
    /// * `offset` - An optional parameter to specify the starting point of the list.
    /// * `limit` - An optional parameter to specify the maximum number of clients to return.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Page` of `ClientOverview` on success, or a `UnifiError` on failure.
    pub async fn list_clients(
        &self,
        site_id: Uuid,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Page<ClientOverview>, UnifiError> {
        let url = format!("{}/v1/sites/{}/clients", self.base_url, site_id);
        let response = self
            .client
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
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub(crate) status_code: u16,
    pub(crate) message: String,
}
