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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_poll_interval_duration() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 9898,
            poll_interval: 30,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 5,
        };

        assert_eq!(config.poll_interval_duration(), Duration::from_secs(30));
    }

    #[test]
    fn test_http_timeout_duration() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 9898,
            poll_interval: 10,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 15,
        };

        assert_eq!(config.http_timeout_duration(), Duration::from_secs(15));
    }

    #[test]
    fn test_metrics_bind_address() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 3000,
            poll_interval: 10,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 5,
        };

        assert_eq!(config.metrics_bind_address(), "0.0.0.0:3000");
    }

    #[test]
    fn test_homewizard_url() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 9898,
            poll_interval: 10,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 5,
        };

        assert_eq!(config.homewizard_url(), "http://192.168.1.100/api/v1/data");
    }

    #[test]
    fn test_homewizard_url_with_hostname() {
        let config = Config {
            host: "homewizard.local".to_string(),
            port: 9898,
            poll_interval: 10,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 5,
        };

        assert_eq!(
            config.homewizard_url(),
            "http://homewizard.local/api/v1/data"
        );
    }

    #[test]
    fn test_config_with_api_token() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 9898,
            poll_interval: 10,
            log_level: "debug".to_string(),
            api_token: Some("secret_token".to_string()),
            http_timeout: 5,
        };

        assert_eq!(config.api_token, Some("secret_token".to_string()));
        assert_eq!(config.log_level, "debug");
    }

    #[test]
    fn test_config_without_api_token() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 9898,
            poll_interval: 10,
            log_level: "info".to_string(),
            api_token: None,
            http_timeout: 5,
        };

        assert_eq!(config.api_token, None);
    }

    #[test]
    fn test_config_edge_cases() {
        let config = Config {
            host: "192.168.1.100".to_string(),
            port: 1,
            poll_interval: 1,
            log_level: "trace".to_string(),
            api_token: Some("".to_string()),
            http_timeout: 1,
        };

        assert_eq!(config.port, 1);
        assert_eq!(config.poll_interval, 1);
        assert_eq!(config.http_timeout, 1);
        assert_eq!(config.metrics_bind_address(), "0.0.0.0:1");
        assert_eq!(config.poll_interval_duration(), Duration::from_secs(1));
        assert_eq!(config.http_timeout_duration(), Duration::from_secs(1));
    }
}
