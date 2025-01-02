use std::collections::HashMap;
use anyhow::Result;
use k8s_openapi::api::core::v1::Pod;
use tokio::time;
use tracing::{info, warn};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    config::Config,
    kubernetes::client::Client,
    metrics::{
        types::{PodMetrics, ContainerMetrics, VolumeMetrics, PodMetricsResource},
        parsers::{parse_cpu, parse_memory},
    },
};

pub struct MetricsCollector {
    client: Client,
    config: Config,
}

impl MetricsCollector {
    pub fn new(client: Client, config: Config) -> Self {
        Self { client, config }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting metrics collection...");
        let mut interval = time::interval(self.config.collect_interval());

        loop {
            interval.tick().await;
            match self.collect_all_metrics().await {
                Ok(metrics) => {
                    if let Ok(json) = serde_json::to_string_pretty(&metrics) {
                        info!("Metrics JSON: {}", json);
                    }
                }
                Err(e) => warn!("Failed to collect metrics: {}", e),
            }
        }
    }

    async fn collect_all_metrics(&self) -> Result<Vec<PodMetrics>> {
        info!("Requesting metrics from namespace: {}", self.config.namespace);
        let pods = self.client.list_pods().await?;
        let metrics_response = self.client.get_metrics().await?;
        info!("Got metrics response with {} items", metrics_response.items.len());
        
        let metrics_map: HashMap<String, PodMetricsResource> = metrics_response
            .items
            .into_iter()
            .filter_map(|m| {
                m.metadata
                    .name
                    .clone()
                    .map(|name| (name, m))
            })
            .collect();

        let mut results = Vec::new();
        for pod in pods.chunks(self.config.batch_size) {
            for pod in pod {
                if let Some(metrics) = self.process_pod_metrics(pod, &metrics_map).await? {
                    results.push(metrics);
                }
            }
        }

        Ok(results)
    }

    async fn process_pod_metrics(
        &self,
        pod: &Pod,
        metrics_map: &HashMap<String, PodMetricsResource>,
    ) -> Result<Option<PodMetrics>> {
        let pod_name = pod.metadata.name.clone().unwrap_or_default();
        
        if let Some(pod_metrics) = metrics_map.get(&pod_name) {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            let mut total_cpu = 0.0;
            let mut total_memory = 0.0;
            let mut containers = Vec::new();

            for container in &pod_metrics.containers {
                let cpu_usage = parse_cpu(&container.usage.cpu)?;
                let memory_usage = parse_memory(&container.usage.memory)?;

                total_cpu += cpu_usage;
                total_memory += memory_usage;

                let volumes = self.collect_volume_metrics(pod);

                containers.push(ContainerMetrics {
                    container_name: container.name.clone(),
                    cpu_usage: Some(cpu_usage),
                    cpu_usage_millicores: Some(format!("{}m", (cpu_usage * 1000.0) as i32)),
                    memory_usage: Some(memory_usage),
                    memory_usage_formatted: Some(format!("{}Mi", (memory_usage / (1024.0 * 1024.0)) as i32)),
                    volumes: volumes.clone(),
                });
            }

            return Ok(Some(PodMetrics {
                timestamp,
                pod_name,
                namespace: pod.metadata.namespace.clone().unwrap_or_default(),
                total_cpu_usage: total_cpu,
                total_cpu_usage_millicores: format!("{}m", (total_cpu * 1000.0) as i32),
                total_memory_usage: total_memory,
                total_memory_usage_formatted: format!("{}Mi", (total_memory / (1024.0 * 1024.0)) as i32),
                containers,
            }));
        }

        Ok(None)
    }

    fn collect_volume_metrics(&self, pod: &Pod) -> Vec<VolumeMetrics> {
        let mut volumes = Vec::new();
        
        if let Some(pod_spec) = &pod.spec {
            if let Some(pod_volumes) = &pod_spec.volumes {
                for volume in pod_volumes {
                    let volume_type = if volume.empty_dir.is_some() {
                        "emptyDir"
                    } else if volume.persistent_volume_claim.is_some() {
                        "persistentVolumeClaim"
                    } else if volume.config_map.is_some() {
                        "configMap"
                    } else if volume.secret.is_some() {
                        "secret"
                    } else if volume.host_path.is_some() {
                        "hostPath"
                    } else {
                        "unknown"
                    };

                    volumes.push(VolumeMetrics {
                        name: volume.name.clone(),
                        capacity_bytes: None, // TODO: Implement volume metrics collection
                        used_bytes: None,
                        volume_type: volume_type.to_string(),
                    });
                }
            }
        }

        volumes
    }
}

// Helper function to get volume metrics (placeholder for future implementation)
fn get_volume_metrics(_pod: &Pod, _volume_name: &str) -> (Option<f64>, Option<f64>) {
    (None, None)
}