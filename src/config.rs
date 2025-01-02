use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default = "default_interval")]
    pub collect_interval_secs: u64,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: u32,
    #[serde(default = "default_namespace")]
    pub namespace: String,
}

fn default_batch_size() -> usize { 50 }
fn default_interval() -> u64 { 60 }
fn default_retry_attempts() -> u32 { 3 }
fn default_namespace() -> String { "default".to_string() }

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("METRICS_COLLECTOR"))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn collect_interval(&self) -> Duration {
        Duration::from_secs(self.collect_interval_secs)
    }
}