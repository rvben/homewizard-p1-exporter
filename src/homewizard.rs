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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_homewizard_client_creation() {
        let client = HomeWizardClient::new(
            "http://192.168.1.100/api/v1/data".to_string(),
            Duration::from_secs(5),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_homewizard_client_creation_with_different_timeout() {
        let client = HomeWizardClient::new(
            "http://192.168.1.100/api/v1/data".to_string(),
            Duration::from_secs(30),
        );
        assert!(client.is_ok());
    }

    #[test]
    fn test_homewizard_error_display() {
        let error = HomeWizardError::ParseError("Invalid JSON".to_string());
        assert_eq!(error.to_string(), "Failed to parse response: Invalid JSON");
    }

    #[test]
    fn test_homewizard_data_deserialization() {
        let json_data = r#"
        {
            "wifi_ssid": "HomeNetwork",
            "wifi_strength": 75.5,
            "smr_version": 50,
            "meter_model": "ISKRA 2M550T-1012",
            "unique_id": "3c39e7aabbccddee",
            "active_tariff": 1,
            "total_power_import_kwh": 1234.567,
            "total_power_import_t1_kwh": 800.123,
            "total_power_import_t2_kwh": 434.444,
            "total_power_export_kwh": 89.012,
            "total_power_export_t1_kwh": 60.789,
            "total_power_export_t2_kwh": 28.223,
            "active_power_w": 1500.0,
            "active_power_l1_w": 1500.0,
            "active_current_a": 6.8,
            "active_current_l1_a": 6.8,
            "voltage_sag_l1_count": 2.0,
            "voltage_swell_l1_count": 1.0,
            "any_power_fail_count": 5.0,
            "long_power_fail_count": 0.0,
            "total_gas_m3": 567.890,
            "gas_timestamp": 1234567890,
            "gas_unique_id": "aabbccddee112233",
            "external": [
                {
                    "unique_id": "112233aabbccddee",
                    "type": "water_meter",
                    "timestamp": 1234567890,
                    "value": 123.456,
                    "unit": "m3"
                }
            ]
        }
        "#;

        let data: Result<HomeWizardData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "HomeNetwork");
        assert_eq!(data.wifi_strength, 75.5);
        assert_eq!(data.smr_version, 50);
        assert_eq!(data.meter_model, "ISKRA 2M550T-1012");
        assert_eq!(data.unique_id, "3c39e7aabbccddee");
        assert_eq!(data.active_tariff, 1);
        assert_eq!(data.total_power_import_kwh, 1234.567);
        assert_eq!(data.total_power_import_t1_kwh, 800.123);
        assert_eq!(data.total_power_import_t2_kwh, 434.444);
        assert_eq!(data.total_power_export_kwh, 89.012);
        assert_eq!(data.total_power_export_t1_kwh, 60.789);
        assert_eq!(data.total_power_export_t2_kwh, 28.223);
        assert_eq!(data.active_power_w, 1500.0);
        assert_eq!(data.active_power_l1_w, 1500.0);
        assert_eq!(data.active_current_a, 6.8);
        assert_eq!(data.active_current_l1_a, 6.8);
        assert_eq!(data.voltage_sag_l1_count, 2.0);
        assert_eq!(data.voltage_swell_l1_count, 1.0);
        assert_eq!(data.any_power_fail_count, 5.0);
        assert_eq!(data.long_power_fail_count, 0.0);
        assert_eq!(data.total_gas_m3, 567.890);
        assert_eq!(data.gas_timestamp, 1234567890);
        assert_eq!(data.gas_unique_id, "aabbccddee112233");
        assert_eq!(data.external.len(), 1);

        let external = &data.external[0];
        assert_eq!(external.unique_id, "112233aabbccddee");
        assert_eq!(external.sensor_type, "water_meter");
        assert_eq!(external.timestamp, 1234567890);
        assert_eq!(external.value, 123.456);
        assert_eq!(external.unit, "m3");
    }

    #[test]
    fn test_homewizard_data_deserialization_minimal() {
        let json_data = r#"
        {
            "wifi_ssid": "Test",
            "wifi_strength": 50.0,
            "smr_version": 40,
            "meter_model": "Test Model",
            "unique_id": "test123",
            "active_tariff": 2,
            "total_power_import_kwh": 100.0,
            "total_power_import_t1_kwh": 60.0,
            "total_power_import_t2_kwh": 40.0,
            "total_power_export_kwh": 10.0,
            "total_power_export_t1_kwh": 6.0,
            "total_power_export_t2_kwh": 4.0,
            "active_power_w": 500.0,
            "active_power_l1_w": 500.0,
            "active_current_a": 2.3,
            "active_current_l1_a": 2.3,
            "voltage_sag_l1_count": 0.0,
            "voltage_swell_l1_count": 0.0,
            "any_power_fail_count": 0.0,
            "long_power_fail_count": 0.0,
            "total_gas_m3": 50.0,
            "gas_timestamp": 1234567890,
            "gas_unique_id": "gas123",
            "external": []
        }
        "#;

        let data: Result<HomeWizardData, _> = serde_json::from_str(json_data);
        assert!(data.is_ok());

        let data = data.unwrap();
        assert_eq!(data.wifi_ssid, "Test");
        assert_eq!(data.active_tariff, 2);
        assert_eq!(data.external.len(), 0);
    }

    #[test]
    fn test_external_sensor_deserialization() {
        let json_data = r#"
        {
            "unique_id": "sensor123",
            "type": "temperature",
            "timestamp": 1234567890,
            "value": 23.5,
            "unit": "°C"
        }
        "#;

        let sensor: Result<ExternalSensor, _> = serde_json::from_str(json_data);
        assert!(sensor.is_ok());

        let sensor = sensor.unwrap();
        assert_eq!(sensor.unique_id, "sensor123");
        assert_eq!(sensor.sensor_type, "temperature");
        assert_eq!(sensor.timestamp, 1234567890);
        assert_eq!(sensor.value, 23.5);
        assert_eq!(sensor.unit, "°C");
    }

    #[test]
    fn test_homewizard_data_clone() {
        let data = HomeWizardData {
            wifi_ssid: "Test".to_string(),
            wifi_strength: 50.0,
            smr_version: 40,
            meter_model: "Test Model".to_string(),
            unique_id: "test123".to_string(),
            active_tariff: 1,
            total_power_import_kwh: 100.0,
            total_power_import_t1_kwh: 60.0,
            total_power_import_t2_kwh: 40.0,
            total_power_export_kwh: 10.0,
            total_power_export_t1_kwh: 6.0,
            total_power_export_t2_kwh: 4.0,
            active_power_w: 500.0,
            active_power_l1_w: 500.0,
            active_current_a: 2.3,
            active_current_l1_a: 2.3,
            voltage_sag_l1_count: 0.0,
            voltage_swell_l1_count: 0.0,
            any_power_fail_count: 0.0,
            long_power_fail_count: 0.0,
            total_gas_m3: 50.0,
            gas_timestamp: 1234567890,
            gas_unique_id: "gas123".to_string(),
            external: vec![],
        };

        let cloned = data.clone();
        assert_eq!(data.wifi_ssid, cloned.wifi_ssid);
        assert_eq!(data.unique_id, cloned.unique_id);
        assert_eq!(data.total_power_import_kwh, cloned.total_power_import_kwh);
    }

    #[test]
    fn test_external_sensor_clone() {
        let sensor = ExternalSensor {
            unique_id: "sensor123".to_string(),
            sensor_type: "temperature".to_string(),
            timestamp: 1234567890,
            value: 23.5,
            unit: "°C".to_string(),
        };

        let cloned = sensor.clone();
        assert_eq!(sensor.unique_id, cloned.unique_id);
        assert_eq!(sensor.sensor_type, cloned.sensor_type);
        assert_eq!(sensor.value, cloned.value);
    }

    #[test]
    fn test_homewizard_error_from_reqwest() {
        // Create a reqwest error by making a request to an invalid URL
        let client = reqwest::Client::new();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            let result = client
                .get("http://invalid-url-that-does-not-exist.test")
                .send()
                .await;
            assert!(result.is_err());

            let reqwest_error = result.unwrap_err();
            let hw_error = HomeWizardError::from(reqwest_error);

            match hw_error {
                HomeWizardError::RequestFailed(_) => {
                    // This is expected
                }
                _ => panic!("Expected RequestFailed error"),
            }
        });
    }
}
