use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// HomeWizard P1 Meter IP address or hostname
    #[arg(long, env = "HOMEWIZARD_HOST")]
    pub host: String,

    /// Port to expose Prometheus metrics on
    #[arg(long, env = "METRICS_PORT", default_value = "9898")]
    pub port: u16,

    /// Interval in seconds between polling the HomeWizard API
    #[arg(long, env = "POLL_INTERVAL", default_value = "10")]
    pub poll_interval: u64,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, env = "LOG_LEVEL", default_value = "info")]
    pub log_level: String,

    /// Optional API token for HomeWizard API v2
    #[arg(long, env = "HOMEWIZARD_API_TOKEN")]
    pub api_token: Option<String>,

    /// Timeout in seconds for HTTP requests to HomeWizard
    #[arg(long, env = "HTTP_TIMEOUT", default_value = "5")]
    pub http_timeout: u64,
}

impl Config {
    pub fn poll_interval_duration(&self) -> Duration {
        Duration::from_secs(self.poll_interval)
    }

    pub fn http_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.http_timeout)
    }

    pub fn metrics_bind_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }

    pub fn homewizard_url(&self) -> String {
        format!("http://{}/api/v1/data", self.host)
    }
}
