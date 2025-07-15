# HomeWizard P1 Prometheus Exporter

[![CI](https://github.com/rvben/homewizard-p1-exporter/actions/workflows/ci.yml/badge.svg)](https://github.com/rvben/homewizard-p1-exporter/actions/workflows/ci.yml)
[![Release](https://github.com/rvben/homewizard-p1-exporter/actions/workflows/release.yml/badge.svg)](https://github.com/rvben/homewizard-p1-exporter/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/homewizard-p1-exporter.svg)](https://crates.io/crates/homewizard-p1-exporter)
[![Docker Pulls](https://img.shields.io/docker/pulls/rvben/homewizard-p1-exporter)](https://hub.docker.com/r/rvben/homewizard-p1-exporter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-blue.svg)](https://www.rust-lang.org)

A Rust-based Prometheus exporter for the HomeWizard P1 Meter, providing real-time electricity and gas consumption metrics.

## Features

- 🚀 **High Performance** - Lightweight and efficient Rust implementation
- 📊 **Real-time Monitoring** - Power consumption/production metrics updated every 10 seconds
- ⚡ **Power Quality Tracking** - Voltage sags/swells and power failure monitoring
- 🔥 **Gas Consumption** - Integrated gas meter reading support
- 💰 **Tariff Support** - Separate metrics for peak (T1) and off-peak (T2) tariffs
- 📡 **Network Monitoring** - WiFi signal strength tracking
- 🐳 **Docker Ready** - Multi-platform images for easy deployment
- ✅ **Production Ready** - Comprehensive test coverage and error handling

## Prerequisites

- HomeWizard P1 Meter with local API enabled
- Rust 1.88+ (for building from source)
- Docker (for container deployment)

## Quick Start

```bash
# Using Docker
docker run -d -p 9898:9898 -e HOMEWIZARD_HOST=192.168.1.100 rvben/homewizard-p1-exporter:latest

# Or using pre-built binary
wget https://github.com/rvben/homewizard-p1-exporter/releases/latest/download/homewizard-p1-exporter-$(uname -m)-linux.tar.gz
tar -xzf homewizard-p1-exporter-*.tar.gz
HOMEWIZARD_HOST=192.168.1.100 ./homewizard-p1-exporter
```

## Installation

### Using Docker (Recommended)

```bash
# From Docker Hub
docker run -d \
  --name homewizard-p1-exporter \
  -p 9898:9898 \
  -e HOMEWIZARD_HOST=192.168.1.100 \
  rvben/homewizard-p1-exporter:latest

# From GitHub Container Registry
docker run -d \
  --name homewizard-p1-exporter \
  -p 9898:9898 \
  -e HOMEWIZARD_HOST=192.168.1.100 \
  ghcr.io/rvben/homewizard-p1-exporter:latest
```

### Using Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/rvben/homewizard-p1-exporter/releases).

```bash
# Example for Linux x86_64
wget https://github.com/rvben/homewizard-p1-exporter/releases/latest/download/homewizard-p1-exporter-x86_64-linux.tar.gz
tar -xzf homewizard-p1-exporter-x86_64-linux.tar.gz
chmod +x homewizard-p1-exporter
HOMEWIZARD_HOST=192.168.1.100 ./homewizard-p1-exporter
```

### Using Cargo

```bash
cargo install homewizard-p1-exporter
HOMEWIZARD_HOST=192.168.1.100 homewizard-p1-exporter
```

### Building from Source

```bash
# Clone the repository
git clone https://github.com/rvben/homewizard-p1-exporter
cd homewizard-p1-exporter

# Build the binary
cargo build --release

# Run the exporter
HOMEWIZARD_HOST=192.168.1.100 ./target/release/homewizard-p1-exporter
```

## Configuration

The exporter can be configured via command-line arguments or environment variables:

| Environment Variable | CLI Flag | Default | Description |
|---------------------|----------|---------|-------------|
| `HOMEWIZARD_HOST` | `--host` | Required | IP address or hostname of HomeWizard P1 Meter |
| `METRICS_PORT` | `--port` | `9898` | Port to expose Prometheus metrics |
| `POLL_INTERVAL` | `--poll-interval` | `10` | Seconds between API polls |
| `LOG_LEVEL` | `--log-level` | `info` | Log level (trace, debug, info, warn, error) |
| `HTTP_TIMEOUT` | `--http-timeout` | `5` | HTTP request timeout in seconds |

## Metrics

The exporter provides the following Prometheus metrics:

| Metric | Type | Description |
|--------|------|-------------|
| `homewizard_p1_power_import_total_kwh` | Counter | Total power imported in kWh |
| `homewizard_p1_power_import_tariff_kwh{tariff}` | Counter | Power imported per tariff (1/2) |
| `homewizard_p1_power_export_total_kwh` | Counter | Total power exported in kWh |
| `homewizard_p1_power_export_tariff_kwh{tariff}` | Counter | Power exported per tariff (1/2) |
| `homewizard_p1_active_power_watts` | Gauge | Current active power in watts |
| `homewizard_p1_active_power_l1_watts` | Gauge | Current active power L1 in watts |
| `homewizard_p1_active_current_amperes` | Gauge | Current active current in amperes |
| `homewizard_p1_active_tariff` | Gauge | Currently active tariff (1 or 2) |
| `homewizard_p1_gas_total_m3` | Counter | Total gas consumption in m³ |
| `homewizard_p1_gas_timestamp` | Gauge | Timestamp of last gas meter reading |
| `homewizard_p1_gas_meter_info{unique_id}` | Gauge | Gas meter information |
| `homewizard_p1_wifi_strength_percent` | Gauge | WiFi signal strength percentage |
| `homewizard_p1_voltage_sag_count_total` | Counter | Total voltage sag events |
| `homewizard_p1_voltage_swell_count_total` | Counter | Total voltage swell events |
| `homewizard_p1_power_failures_any_total` | Counter | Total power failures |
| `homewizard_p1_power_failures_long_total` | Counter | Total long power failures |
| `homewizard_p1_meter_info{meter_id,meter_model,smr_version,wifi_ssid}` | Gauge | Meter information |
| `homewizard_p1_external_sensor_value{unique_id,type,unit}` | Gauge | External sensor value |
| `homewizard_p1_external_sensor_timestamp{unique_id,type}` | Gauge | External sensor timestamp |

## Prometheus Configuration

Add the following to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'homewizard'
    static_configs:
      - targets: ['localhost:9898']
    scrape_interval: 30s
```

## Enabling HomeWizard Local API

1. Open the HomeWizard Energy app
2. Go to Settings → Meters → Your P1 Meter
3. Enable "Local API"

## Grafana Dashboard

An example Grafana dashboard is included in `grafana-dashboard.json`. To import:

1. Open Grafana
2. Go to Dashboards → Import
3. Upload the JSON file or paste its contents
4. Select your Prometheus data source
5. Click Import

The dashboard includes:
- Real-time power usage graph
- Current power gauge
- Total power imported/exported
- Gas consumption
- Current tariff indicator

## Development

```bash
# Show all available make targets
make help

# Build the binary
make build

# Run tests
make test

# Check code formatting and linting
make check

# Run the exporter locally
HOMEWIZARD_HOST=192.168.1.100 make run

# Build Docker image
make docker-build

# Run in Docker
HOMEWIZARD_HOST=192.168.1.100 make docker-run
```

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.