use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Opts, Registry, TextEncoder, Encoder
};
use anyhow::Result;
use crate::homewizard::HomeWizardData;

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
    
    // Network metrics
    wifi_strength: Gauge,
    
    // Power quality metrics
    voltage_sag_count: Counter,
    voltage_swell_count: Counter,
    power_failures_any: Counter,
    power_failures_long: Counter,
    
    // Info metric
    meter_info: GaugeVec,
    
    registry: Registry,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();
        
        // Power import metrics
        let power_import_total = Counter::with_opts(
            Opts::new("homewizard_p1_power_import_total_kwh", "Total power imported in kWh")
        )?;
        registry.register(Box::new(power_import_total.clone()))?;
        
        let power_import_tariff = CounterVec::new(
            Opts::new("homewizard_p1_power_import_tariff_kwh", "Power imported per tariff in kWh"),
            &["tariff"]
        )?;
        registry.register(Box::new(power_import_tariff.clone()))?;
        
        // Power export metrics
        let power_export_total = Counter::with_opts(
            Opts::new("homewizard_p1_power_export_total_kwh", "Total power exported in kWh")
        )?;
        registry.register(Box::new(power_export_total.clone()))?;
        
        let power_export_tariff = CounterVec::new(
            Opts::new("homewizard_p1_power_export_tariff_kwh", "Power exported per tariff in kWh"),
            &["tariff"]
        )?;
        registry.register(Box::new(power_export_tariff.clone()))?;
        
        // Current power metrics
        let active_power = Gauge::with_opts(
            Opts::new("homewizard_p1_active_power_watts", "Current active power in watts")
        )?;
        registry.register(Box::new(active_power.clone()))?;
        
        let active_power_l1 = Gauge::with_opts(
            Opts::new("homewizard_p1_active_power_l1_watts", "Current active power L1 in watts")
        )?;
        registry.register(Box::new(active_power_l1.clone()))?;
        
        let active_current = Gauge::with_opts(
            Opts::new("homewizard_p1_active_current_amperes", "Current active current in amperes")
        )?;
        registry.register(Box::new(active_current.clone()))?;
        
        let active_current_l1 = Gauge::with_opts(
            Opts::new("homewizard_p1_active_current_l1_amperes", "Current active current L1 in amperes")
        )?;
        registry.register(Box::new(active_current_l1.clone()))?;
        
        let active_tariff = Gauge::with_opts(
            Opts::new("homewizard_p1_active_tariff", "Currently active tariff (1 or 2)")
        )?;
        registry.register(Box::new(active_tariff.clone()))?;
        
        // Gas metrics
        let gas_total = Counter::with_opts(
            Opts::new("homewizard_p1_gas_total_m3", "Total gas consumption in m3")
        )?;
        registry.register(Box::new(gas_total.clone()))?;
        
        // Network metrics
        let wifi_strength = Gauge::with_opts(
            Opts::new("homewizard_p1_wifi_strength_percent", "WiFi signal strength percentage")
        )?;
        registry.register(Box::new(wifi_strength.clone()))?;
        
        // Power quality metrics
        let voltage_sag_count = Counter::with_opts(
            Opts::new("homewizard_p1_voltage_sag_count_total", "Total voltage sag events")
        )?;
        registry.register(Box::new(voltage_sag_count.clone()))?;
        
        let voltage_swell_count = Counter::with_opts(
            Opts::new("homewizard_p1_voltage_swell_count_total", "Total voltage swell events")
        )?;
        registry.register(Box::new(voltage_swell_count.clone()))?;
        
        let power_failures_any = Counter::with_opts(
            Opts::new("homewizard_p1_power_failures_any_total", "Total power failures (any duration)")
        )?;
        registry.register(Box::new(power_failures_any.clone()))?;
        
        let power_failures_long = Counter::with_opts(
            Opts::new("homewizard_p1_power_failures_long_total", "Total long power failures")
        )?;
        registry.register(Box::new(power_failures_long.clone()))?;
        
        // Info metric
        let meter_info = GaugeVec::new(
            Opts::new("homewizard_p1_meter_info", "Meter information"),
            &["meter_id", "meter_model", "smr_version", "wifi_ssid"]
        )?;
        registry.register(Box::new(meter_info.clone()))?;
        
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
            wifi_strength,
            voltage_sag_count,
            voltage_swell_count,
            power_failures_any,
            power_failures_long,
            meter_info,
            registry,
        })
    }
    
    pub fn update(&self, data: &HomeWizardData) -> Result<()> {
        // Update power import metrics
        self.power_import_total.reset();
        self.power_import_total.inc_by(data.total_power_import_kwh);
        
        self.power_import_tariff.reset();
        self.power_import_tariff.with_label_values(&["1"]).inc_by(data.total_power_import_t1_kwh);
        self.power_import_tariff.with_label_values(&["2"]).inc_by(data.total_power_import_t2_kwh);
        
        // Update power export metrics
        self.power_export_total.reset();
        self.power_export_total.inc_by(data.total_power_export_kwh);
        
        self.power_export_tariff.reset();
        self.power_export_tariff.with_label_values(&["1"]).inc_by(data.total_power_export_t1_kwh);
        self.power_export_tariff.with_label_values(&["2"]).inc_by(data.total_power_export_t2_kwh);
        
        // Update current power metrics
        self.active_power.set(data.active_power_w);
        self.active_power_l1.set(data.active_power_l1_w);
        self.active_current.set(data.active_current_a);
        self.active_current_l1.set(data.active_current_l1_a);
        self.active_tariff.set(data.active_tariff as f64);
        
        // Update gas metrics
        self.gas_total.reset();
        self.gas_total.inc_by(data.total_gas_m3);
        
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
        self.meter_info.with_label_values(&[
            &data.unique_id,
            &data.meter_model,
            &data.smr_version.to_string(),
            &data.wifi_ssid,
        ]).set(1.0);
        
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