use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PodMetrics {
    pub timestamp: u64,
    pub pod_name: String,
    pub namespace: String,
    pub total_cpu_usage: f64,
    pub total_cpu_usage_millicores: String,
    pub total_memory_usage: f64,
    pub total_memory_usage_formatted: String,
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerMetrics {
    pub container_name: String,
    pub cpu_usage: Option<f64>,
    pub cpu_usage_millicores: Option<String>,
    pub memory_usage: Option<f64>,
    pub memory_usage_formatted: Option<String>,
    pub volumes: Vec<VolumeMetrics>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeMetrics {
    pub name: String,
    pub capacity_bytes: Option<f64>,
    pub used_bytes: Option<f64>,
    pub volume_type: String,
}

#[derive(Debug, Deserialize)]
pub struct MetricsResponse {
    pub items: Vec<PodMetricsResource>,
}

#[derive(Debug, Deserialize)]
pub struct PodMetricsResource {
    pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub containers: Vec<ContainerMetricsResource>,
}

#[derive(Debug, Deserialize)]
pub struct ContainerMetricsResource {
    pub name: String,
    pub usage: ResourceMetrics,
}

#[derive(Debug, Deserialize)]
pub struct ResourceMetrics {
    pub cpu: String,
    pub memory: String,
}