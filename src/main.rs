use anyhow::Result;
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;
use crate::kubernetes::client::Client;
use crate::metrics::collector::MetricsCollector;

mod config;
mod kubernetes;
mod metrics;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Initialize rustls crypto (required for kube client in newer versions)
    let _ = rustls::crypto::ring::default_provider().install_default();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Load configuration
    let config = config::Config::new()?;

    // Initialize Kubernetes client
    let client = Client::new(config.namespace.clone()).await?;

    // Create and start metrics collector
    let collector = MetricsCollector::new(client, config);
    collector.start().await?;

    Ok(())
}