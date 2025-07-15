mod config;
mod homewizard;
mod metrics;

use anyhow::Result;
use axum::{Router, routing::get};
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
    let client = HomeWizardClient::new(config.homewizard_url(), config.http_timeout_duration())?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;

    fn create_test_app() -> Router {
        let shared_metrics: SharedMetrics = Arc::new(RwLock::new(
            "# HELP test_metric A test metric\n# TYPE test_metric counter\ntest_metric 42\n"
                .to_string(),
        ));

        Router::new()
            .route("/metrics", get(metrics_handler))
            .route("/health", get(health_handler))
            .route("/", get(root_handler))
            .with_state(shared_metrics)
    }

    #[tokio::test]
    async fn test_health_handler() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "OK");
    }

    #[tokio::test]
    async fn test_root_handler() {
        let app = create_test_app();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("HomeWizard P1 Prometheus Exporter"));
        assert!(body_str.contains("/metrics"));
        assert!(body_str.contains("/health"));
    }

    #[tokio::test]
    async fn test_metrics_handler() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("test_metric"));
        assert!(body_str.contains("42"));
    }

    #[tokio::test]
    async fn test_metrics_handler_with_empty_metrics() {
        let shared_metrics: SharedMetrics = Arc::new(RwLock::new(String::new()));
        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .with_state(shared_metrics);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(body, "");
    }

    #[tokio::test]
    async fn test_not_found_route() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_metrics_handler_concurrent_access() {
        let shared_metrics: SharedMetrics = Arc::new(RwLock::new(
            "# HELP test_metric A test metric\n# TYPE test_metric counter\ntest_metric 42\n"
                .to_string(),
        ));

        // Make multiple concurrent requests
        let mut handles = Vec::new();
        for _ in 0..10 {
            let app = Router::new()
                .route("/metrics", get(metrics_handler))
                .with_state(shared_metrics.clone());

            let handle = tokio::spawn(async move {
                let response = app
                    .oneshot(
                        Request::builder()
                            .uri("/metrics")
                            .body(Body::empty())
                            .unwrap(),
                    )
                    .await
                    .unwrap();

                assert_eq!(response.status(), StatusCode::OK);
                let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                    .await
                    .unwrap();
                String::from_utf8(body.to_vec()).unwrap()
            });
            handles.push(handle);
        }

        // Wait for all requests to complete
        for handle in handles {
            let body = handle.await.unwrap();
            assert!(body.contains("test_metric"));
        }
    }

    #[tokio::test]
    async fn test_health_handler_method_not_allowed() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_metrics_update_during_request() {
        let shared_metrics: SharedMetrics = Arc::new(RwLock::new("initial_metric 1\n".to_string()));

        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .with_state(shared_metrics.clone());

        // Get initial metrics
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("initial_metric 1"));

        // Update metrics
        {
            let mut metrics_guard = shared_metrics.write().await;
            *metrics_guard = "updated_metric 2\n".to_string();
        }

        // Get updated metrics
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("updated_metric 2"));
    }

    #[test]
    fn test_shared_metrics_type_alias() {
        let shared_metrics: SharedMetrics = Arc::new(RwLock::new("test".to_string()));

        // Test that the type alias works correctly
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let guard = shared_metrics.read().await;
            assert_eq!(*guard, "test");
        });
    }
}
