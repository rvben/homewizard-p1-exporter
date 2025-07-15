use crate::homewizard::HomeWizardData;
use anyhow::Result;
use prometheus::{Counter, CounterVec, Encoder, Gauge, GaugeVec, Opts, Registry, TextEncoder};

pub struct Metrics {
    // Power import metrics
    power_import_total: Counter,
    power_import_tariff: CounterVec,

    // Power export metrics
    power_export_total: Counter,
    power_export_tariff: CounterVec,

    // Current power metrics
    active_power: Gauge,
    active_power_l1: Gauge,
    active_current: Gauge,
    active_current_l1: Gauge,
    active_tariff: Gauge,

    // Gas metrics
    gas_total: Counter,
    gas_timestamp: Gauge,
    gas_meter_info: GaugeVec,

    // Network metrics
    wifi_strength: Gauge,

    // Power quality metrics
    voltage_sag_count: Counter,
    voltage_swell_count: Counter,
    power_failures_any: Counter,
    power_failures_long: Counter,

    // Info metric
    meter_info: GaugeVec,

    // External sensors
    external_sensor_value: GaugeVec,
    external_sensor_timestamp: GaugeVec,

    registry: Registry,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        // Power import metrics
        let power_import_total = Counter::with_opts(Opts::new(
            "homewizard_p1_power_import_total_kwh",
            "Total power imported in kWh",
        ))?;
        registry.register(Box::new(power_import_total.clone()))?;

        let power_import_tariff = CounterVec::new(
            Opts::new(
                "homewizard_p1_power_import_tariff_kwh",
                "Power imported per tariff in kWh",
            ),
            &["tariff"],
        )?;
        registry.register(Box::new(power_import_tariff.clone()))?;

        // Power export metrics
        let power_export_total = Counter::with_opts(Opts::new(
            "homewizard_p1_power_export_total_kwh",
            "Total power exported in kWh",
        ))?;
        registry.register(Box::new(power_export_total.clone()))?;

        let power_export_tariff = CounterVec::new(
            Opts::new(
                "homewizard_p1_power_export_tariff_kwh",
                "Power exported per tariff in kWh",
            ),
            &["tariff"],
        )?;
        registry.register(Box::new(power_export_tariff.clone()))?;

        // Current power metrics
        let active_power = Gauge::with_opts(Opts::new(
            "homewizard_p1_active_power_watts",
            "Current active power in watts",
        ))?;
        registry.register(Box::new(active_power.clone()))?;

        let active_power_l1 = Gauge::with_opts(Opts::new(
            "homewizard_p1_active_power_l1_watts",
            "Current active power L1 in watts",
        ))?;
        registry.register(Box::new(active_power_l1.clone()))?;

        let active_current = Gauge::with_opts(Opts::new(
            "homewizard_p1_active_current_amperes",
            "Current active current in amperes",
        ))?;
        registry.register(Box::new(active_current.clone()))?;

        let active_current_l1 = Gauge::with_opts(Opts::new(
            "homewizard_p1_active_current_l1_amperes",
            "Current active current L1 in amperes",
        ))?;
        registry.register(Box::new(active_current_l1.clone()))?;

        let active_tariff = Gauge::with_opts(Opts::new(
            "homewizard_p1_active_tariff",
            "Currently active tariff (1 or 2)",
        ))?;
        registry.register(Box::new(active_tariff.clone()))?;

        // Gas metrics
        let gas_total = Counter::with_opts(Opts::new(
            "homewizard_p1_gas_total_m3",
            "Total gas consumption in m3",
        ))?;
        registry.register(Box::new(gas_total.clone()))?;

        let gas_timestamp = Gauge::with_opts(Opts::new(
            "homewizard_p1_gas_timestamp",
            "Timestamp of last gas meter reading",
        ))?;
        registry.register(Box::new(gas_timestamp.clone()))?;

        let gas_meter_info = GaugeVec::new(
            Opts::new("homewizard_p1_gas_meter_info", "Gas meter information"),
            &["unique_id"],
        )?;
        registry.register(Box::new(gas_meter_info.clone()))?;

        // Network metrics
        let wifi_strength = Gauge::with_opts(Opts::new(
            "homewizard_p1_wifi_strength_percent",
            "WiFi signal strength percentage",
        ))?;
        registry.register(Box::new(wifi_strength.clone()))?;

        // Power quality metrics
        let voltage_sag_count = Counter::with_opts(Opts::new(
            "homewizard_p1_voltage_sag_count_total",
            "Total voltage sag events",
        ))?;
        registry.register(Box::new(voltage_sag_count.clone()))?;

        let voltage_swell_count = Counter::with_opts(Opts::new(
            "homewizard_p1_voltage_swell_count_total",
            "Total voltage swell events",
        ))?;
        registry.register(Box::new(voltage_swell_count.clone()))?;

        let power_failures_any = Counter::with_opts(Opts::new(
            "homewizard_p1_power_failures_any_total",
            "Total power failures (any duration)",
        ))?;
        registry.register(Box::new(power_failures_any.clone()))?;

        let power_failures_long = Counter::with_opts(Opts::new(
            "homewizard_p1_power_failures_long_total",
            "Total long power failures",
        ))?;
        registry.register(Box::new(power_failures_long.clone()))?;

        // Info metric
        let meter_info = GaugeVec::new(
            Opts::new("homewizard_p1_meter_info", "Meter information"),
            &["meter_id", "meter_model", "smr_version", "wifi_ssid"],
        )?;
        registry.register(Box::new(meter_info.clone()))?;

        // External sensors
        let external_sensor_value = GaugeVec::new(
            Opts::new(
                "homewizard_p1_external_sensor_value",
                "External sensor value",
            ),
            &["unique_id", "type", "unit"],
        )?;
        registry.register(Box::new(external_sensor_value.clone()))?;

        let external_sensor_timestamp = GaugeVec::new(
            Opts::new(
                "homewizard_p1_external_sensor_timestamp",
                "External sensor timestamp",
            ),
            &["unique_id", "type"],
        )?;
        registry.register(Box::new(external_sensor_timestamp.clone()))?;

        Ok(Self {
            power_import_total,
            power_import_tariff,
            power_export_total,
            power_export_tariff,
            active_power,
            active_power_l1,
            active_current,
            active_current_l1,
            active_tariff,
            gas_total,
            gas_timestamp,
            gas_meter_info,
            wifi_strength,
            voltage_sag_count,
            voltage_swell_count,
            power_failures_any,
            power_failures_long,
            meter_info,
            external_sensor_value,
            external_sensor_timestamp,
            registry,
        })
    }

    pub fn update(&self, data: &HomeWizardData) -> Result<()> {
        // Update power import metrics
        self.power_import_total.reset();
        self.power_import_total.inc_by(data.total_power_import_kwh);

        self.power_import_tariff.reset();
        self.power_import_tariff
            .with_label_values(&["1"])
            .inc_by(data.total_power_import_t1_kwh);
        self.power_import_tariff
            .with_label_values(&["2"])
            .inc_by(data.total_power_import_t2_kwh);

        // Update power export metrics
        self.power_export_total.reset();
        self.power_export_total.inc_by(data.total_power_export_kwh);

        self.power_export_tariff.reset();
        self.power_export_tariff
            .with_label_values(&["1"])
            .inc_by(data.total_power_export_t1_kwh);
        self.power_export_tariff
            .with_label_values(&["2"])
            .inc_by(data.total_power_export_t2_kwh);

        // Update current power metrics
        self.active_power.set(data.active_power_w);
        self.active_power_l1.set(data.active_power_l1_w);
        self.active_current.set(data.active_current_a);
        self.active_current_l1.set(data.active_current_l1_a);
        self.active_tariff.set(data.active_tariff as f64);

        // Update gas metrics
        self.gas_total.reset();
        self.gas_total.inc_by(data.total_gas_m3);

        // Update gas timestamp
        self.gas_timestamp.set(data.gas_timestamp as f64);

        // Update gas meter info
        self.gas_meter_info.reset();
        self.gas_meter_info
            .with_label_values(&[&data.gas_unique_id])
            .set(1.0);

        // Update network metrics
        self.wifi_strength.set(data.wifi_strength);

        // Update power quality metrics
        self.voltage_sag_count.reset();
        self.voltage_sag_count.inc_by(data.voltage_sag_l1_count);

        self.voltage_swell_count.reset();
        self.voltage_swell_count.inc_by(data.voltage_swell_l1_count);

        self.power_failures_any.reset();
        self.power_failures_any.inc_by(data.any_power_fail_count);

        self.power_failures_long.reset();
        self.power_failures_long.inc_by(data.long_power_fail_count);

        // Update info metric
        self.meter_info.reset();
        self.meter_info
            .with_label_values(&[
                &data.unique_id,
                &data.meter_model,
                &data.smr_version.to_string(),
                &data.wifi_ssid,
            ])
            .set(1.0);

        // Update external sensors
        self.external_sensor_value.reset();
        self.external_sensor_timestamp.reset();
        for sensor in &data.external {
            self.external_sensor_value
                .with_label_values(&[&sensor.unique_id, &sensor.sensor_type, &sensor.unit])
                .set(sensor.value);

            self.external_sensor_timestamp
                .with_label_values(&[&sensor.unique_id, &sensor.sensor_type])
                .set(sensor.timestamp as f64);
        }

        Ok(())
    }

    pub fn gather(&self) -> Result<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::homewizard::{ExternalSensor, HomeWizardData};

    fn create_test_data() -> HomeWizardData {
        HomeWizardData {
            wifi_ssid: "TestNetwork".to_string(),
            wifi_strength: 75.5,
            smr_version: 50,
            meter_model: "ISKRA 2M550T-1012".to_string(),
            unique_id: "3c39e7aabbccddee".to_string(),
            active_tariff: 1,
            total_power_import_kwh: 1234.567,
            total_power_import_t1_kwh: 800.123,
            total_power_import_t2_kwh: 434.444,
            total_power_export_kwh: 89.012,
            total_power_export_t1_kwh: 60.789,
            total_power_export_t2_kwh: 28.223,
            active_power_w: 1500.0,
            active_power_l1_w: 1500.0,
            active_current_a: 6.8,
            active_current_l1_a: 6.8,
            voltage_sag_l1_count: 2.0,
            voltage_swell_l1_count: 1.0,
            any_power_fail_count: 5.0,
            long_power_fail_count: 0.0,
            total_gas_m3: 567.890,
            gas_timestamp: 1234567890,
            gas_unique_id: "aabbccddee112233".to_string(),
            external: vec![
                ExternalSensor {
                    unique_id: "sensor123".to_string(),
                    sensor_type: "temperature".to_string(),
                    timestamp: 1234567890,
                    value: 23.5,
                    unit: "°C".to_string(),
                },
                ExternalSensor {
                    unique_id: "sensor456".to_string(),
                    sensor_type: "water_meter".to_string(),
                    timestamp: 1234567890,
                    value: 123.456,
                    unit: "m3".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics::new();
        assert!(metrics.is_ok());
    }

    #[test]
    fn test_metrics_update() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        let result = metrics.update(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_metrics_gather() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let result = metrics.gather();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.contains("homewizard_p1_power_import_total_kwh"));
        assert!(output.contains("homewizard_p1_active_power_watts"));
        assert!(output.contains("homewizard_p1_gas_total_m3"));
        assert!(output.contains("homewizard_p1_wifi_strength_percent"));
        assert!(output.contains("homewizard_p1_meter_info"));
        assert!(output.contains("homewizard_p1_external_sensor_value"));
    }

    #[test]
    fn test_metrics_power_import_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_power_import_total_kwh 1234.567"));
        assert!(output.contains("homewizard_p1_power_import_tariff_kwh{tariff=\"1\"} 800.123"));
        assert!(output.contains("homewizard_p1_power_import_tariff_kwh{tariff=\"2\"} 434.444"));
    }

    #[test]
    fn test_metrics_power_export_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_power_export_total_kwh 89.012"));
        assert!(output.contains("homewizard_p1_power_export_tariff_kwh{tariff=\"1\"} 60.789"));
        assert!(output.contains("homewizard_p1_power_export_tariff_kwh{tariff=\"2\"} 28.223"));
    }

    #[test]
    fn test_metrics_active_power_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_active_power_watts 1500"));
        assert!(output.contains("homewizard_p1_active_power_l1_watts 1500"));
        assert!(output.contains("homewizard_p1_active_current_amperes 6.8"));
        assert!(output.contains("homewizard_p1_active_current_l1_amperes 6.8"));
        assert!(output.contains("homewizard_p1_active_tariff 1"));
    }

    #[test]
    fn test_metrics_gas_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_gas_total_m3 567.89"));
        assert!(output.contains("homewizard_p1_gas_timestamp 1234567890"));
        assert!(output.contains("homewizard_p1_gas_meter_info{unique_id=\"aabbccddee112233\"} 1"));
    }

    #[test]
    fn test_metrics_wifi_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_wifi_strength_percent 75.5"));
    }

    #[test]
    fn test_metrics_power_quality_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_voltage_sag_count_total 2"));
        assert!(output.contains("homewizard_p1_voltage_swell_count_total 1"));
        assert!(output.contains("homewizard_p1_power_failures_any_total 5"));
        assert!(output.contains("homewizard_p1_power_failures_long_total 0"));
    }

    #[test]
    fn test_metrics_meter_info_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_meter_info{meter_id=\"3c39e7aabbccddee\",meter_model=\"ISKRA 2M550T-1012\",smr_version=\"50\",wifi_ssid=\"TestNetwork\"} 1"));
    }

    #[test]
    fn test_metrics_external_sensors_values() {
        let metrics = Metrics::new().unwrap();
        let data = create_test_data();

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        // Check for sensor values (may be URL encoded)
        assert!(
            output.contains("homewizard_p1_external_sensor_value") && output.contains("sensor123")
        );
        assert!(
            output.contains("homewizard_p1_external_sensor_value") && output.contains("sensor456")
        );
        assert!(
            output.contains("homewizard_p1_external_sensor_timestamp")
                && output.contains("sensor123")
        );
        assert!(
            output.contains("homewizard_p1_external_sensor_timestamp")
                && output.contains("sensor456")
        );

        // Check for values
        assert!(output.contains("23.5"));
        assert!(output.contains("123.456"));
        assert!(output.contains("1234567890"));
    }

    #[test]
    fn test_metrics_with_empty_external_sensors() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.external = vec![];

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(!output.contains("homewizard_p1_external_sensor_value"));
        assert!(!output.contains("homewizard_p1_external_sensor_timestamp"));
    }

    #[test]
    fn test_metrics_with_different_tariff() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.active_tariff = 2;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_active_tariff 2"));
    }

    #[test]
    fn test_metrics_with_zero_values() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_power_import_kwh = 0.0;
        data.total_power_export_kwh = 0.0;
        data.active_power_w = 0.0;
        data.total_gas_m3 = 0.0;
        data.wifi_strength = 0.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_power_import_total_kwh 0"));
        assert!(output.contains("homewizard_p1_power_export_total_kwh 0"));
        assert!(output.contains("homewizard_p1_active_power_watts 0"));
        assert!(output.contains("homewizard_p1_gas_total_m3 0"));
        assert!(output.contains("homewizard_p1_wifi_strength_percent 0"));
    }

    #[test]
    fn test_metrics_update_multiple_times() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();

        // First update
        metrics.update(&data).unwrap();
        let output1 = metrics.gather().unwrap();
        assert!(output1.contains("homewizard_p1_active_power_watts 1500"));

        // Second update with different values
        data.active_power_w = 2000.0;
        metrics.update(&data).unwrap();
        let output2 = metrics.gather().unwrap();
        assert!(output2.contains("homewizard_p1_active_power_watts 2000"));
    }

    #[test]
    fn test_metrics_large_values() {
        let metrics = Metrics::new().unwrap();
        let mut data = create_test_data();
        data.total_power_import_kwh = 999999.999;
        data.active_power_w = 99999.0;

        metrics.update(&data).unwrap();
        let output = metrics.gather().unwrap();

        assert!(output.contains("homewizard_p1_power_import_total_kwh 999999.999"));
        assert!(output.contains("homewizard_p1_active_power_watts 99999"));
    }
}
