use anyhow::Result;
use serde::Deserialize;
use std::{time::Duration, fs};
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_interval")]
    pub collect_interval_secs: u64,
    #[serde(default = "default_namespace")]
    pub namespace: String,
}

fn default_batch_size() -> usize { 50 }
fn default_interval() -> u64 { 30 }
fn default_namespace() -> String { "default".to_string() }

impl Config {
    pub fn new() -> Result<Self> {
        // Try loading from .env file first
        if let Ok(path) = dotenv::dotenv() {
            info!("Loaded configuration from {:?}", path);
        }

        // Then try Kubernetes ConfigMap
        let config = Self::from_configmap()
            .or_else(|_| Self::from_env())?;

        info!("Using configuration: {:?}", config);
        Ok(config)
    }

    fn from_configmap() -> Result<Self> {
        const CONFIG_PATH: &str = "/etc/config";
        
        // Try to read from ConfigMap
        if !std::path::Path::new(CONFIG_PATH).exists() {
            info!("No Kubernetes ConfigMap found at {}", CONFIG_PATH);
            return Err(anyhow::anyhow!("ConfigMap not found"));
        }

        let batch_size = fs::read_to_string(format!("{}/METRICS_COLLECTOR_BATCH_SIZE", CONFIG_PATH))
            .map(|s| s.trim().parse::<usize>())
            .unwrap_or(Ok(default_batch_size()))?;

        let interval = fs::read_to_string(format!("{}/METRICS_COLLECTOR_COLLECT_INTERVAL_SECS", CONFIG_PATH))
            .map(|s| s.trim().parse::<u64>())
            .unwrap_or(Ok(default_interval()))?;

        let namespace = fs::read_to_string(format!("{}/METRICS_COLLECTOR_NAMESPACE", CONFIG_PATH))
            .unwrap_or_else(|_| default_namespace());

        Ok(Self {
            batch_size,
            collect_interval_secs: interval,
            namespace,
        })
    }

    fn from_env() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("METRICS_COLLECTOR"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn collect_interval(&self) -> Duration {
        Duration::from_secs(self.collect_interval_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::new().unwrap();
        assert_eq!(config.batch_size, default_batch_size());
        assert_eq!(config.collect_interval_secs, default_interval());
        assert_eq!(config.namespace, default_namespace());
    }
}