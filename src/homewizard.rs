use anyhow::Result;
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HomeWizardError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct HomeWizardData {
    pub wifi_ssid: String,
    pub wifi_strength: f64,
    pub smr_version: i32,
    pub meter_model: String,
    pub unique_id: String,
    pub active_tariff: i32,
    pub total_power_import_kwh: f64,
    pub total_power_import_t1_kwh: f64,
    pub total_power_import_t2_kwh: f64,
    pub total_power_export_kwh: f64,
    pub total_power_export_t1_kwh: f64,
    pub total_power_export_t2_kwh: f64,
    pub active_power_w: f64,
    pub active_power_l1_w: f64,
    pub active_current_a: f64,
    pub active_current_l1_a: f64,
    pub voltage_sag_l1_count: f64,
    pub voltage_swell_l1_count: f64,
    pub any_power_fail_count: f64,
    pub long_power_fail_count: f64,
    pub total_gas_m3: f64,
    pub gas_timestamp: i64,
    pub gas_unique_id: String,
    pub external: Vec<ExternalSensor>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExternalSensor {
    pub unique_id: String,
    #[serde(rename = "type")]
    pub sensor_type: String,
    pub timestamp: i64,
    pub value: f64,
    pub unit: String,
}

pub struct HomeWizardClient {
    client: reqwest::Client,
    url: String,
}

impl HomeWizardClient {
    pub fn new(url: String, timeout: std::time::Duration) -> Result<Self> {
        let client = reqwest::Client::builder().timeout(timeout).build()?;

        Ok(Self { client, url })
    }

    pub async fn fetch_data(&self) -> Result<HomeWizardData, HomeWizardError> {
        let response = self.client.get(&self.url).send().await?;

        if !response.status().is_success() {
            return Err(HomeWizardError::ParseError(format!(
                "HTTP status: {}",
                response.status()
            )));
        }

        let data = response.json::<HomeWizardData>().await?;
        Ok(data)
    }
}
