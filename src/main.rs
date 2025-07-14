mod config;
mod homewizard;
mod metrics;

use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::homewizard::HomeWizardClient;
use crate::metrics::Metrics;

type SharedMetrics = Arc<RwLock<String>>;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse configuration
    let config = Config::parse();
    
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    info!("Starting HomeWizard P1 Prometheus Exporter");
    info!("HomeWizard host: {}", config.host);
    info!("Metrics port: {}", config.port);
    info!("Poll interval: {}s", config.poll_interval);
    
    // Initialize metrics
    let metrics = Arc::new(Metrics::new()?);
    let shared_metrics: SharedMetrics = Arc::new(RwLock::new(String::new()));
    
    // Initialize HomeWizard client
    let client = HomeWizardClient::new(
        config.homewizard_url(),
        config.http_timeout_duration(),
    )?;
    
    // Start polling task
    let poll_metrics = metrics.clone();
    let poll_shared_metrics = shared_metrics.clone();
    let poll_interval = config.poll_interval_duration();
    
    tokio::spawn(async move {
        let mut interval = interval(poll_interval);
        interval.tick().await; // First tick completes immediately
        
        loop {
            interval.tick().await;
            
            match client.fetch_data().await {
                Ok(data) => {
                    info!("Successfully fetched data from HomeWizard");
                    
                    if let Err(e) = poll_metrics.update(&data) {
                        error!("Failed to update metrics: {}", e);
                        continue;
                    }
                    
                    match poll_metrics.gather() {
                        Ok(metrics_text) => {
                            let mut metrics_guard = poll_shared_metrics.write().await;
                            *metrics_guard = metrics_text;
                        }
                        Err(e) => {
                            error!("Failed to gather metrics: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to fetch data from HomeWizard: {}", e);
                }
            }
        }
    });
    
    // Initialize HTTP server
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_handler))
        .route("/", get(root_handler))
        .with_state(shared_metrics);
    
    let addr = config.metrics_bind_address();
    info!("Starting metrics server on {}", &addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn metrics_handler(
    axum::extract::State(metrics): axum::extract::State<SharedMetrics>,
) -> String {
    let metrics_guard = metrics.read().await;
    metrics_guard.clone()
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn root_handler() -> &'static str {
    "HomeWizard P1 Prometheus Exporter\n\nEndpoints:\n  /metrics - Prometheus metrics\n  /health  - Health check\n"
}