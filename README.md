# Kubernetes Pod Metrics Collector

A Rust-based service that collects CPU, Memory, and Disk metrics from Kubernetes pods. This project provides a robust way to monitor resource utilization across your Kubernetes cluster.

## Features

- Collects CPU usage metrics (cores and millicores)
- Monitors Memory usage (bytes and formatted units)
- Tracks Volume information
- Supports batch processing for large clusters
- Configurable collection intervals
- JSON output format
- Error handling and logging
- Kubernetes ConfigMap support
- Local development with .env file support

## Prerequisites

- Rust (Latest stable version)
- Minikube or access to a Kubernetes cluster
- kubectl configured for your cluster
- Metrics Server enabled in your cluster

## Quick Start

1. Clone the repository:
```bash
git clone 
cd pod-metrics-collector
```

2. Create a .env file for local development:
```bash
cat > .env << EOL
METRICS_COLLECTOR_BATCH_SIZE=10
METRICS_COLLECTOR_COLLECT_INTERVAL_SECS=30
METRICS_COLLECTOR_NAMESPACE=default
EOL
```

3. Build the project:
```bash
cargo build
```

4. Start Minikube (if using locally):
```bash
minikube start
minikube addons enable metrics-server
```

5. Deploy the test load generator:
```bash
kubectl apply -f deployments/load-generator.yaml
```

6. Run the metrics collector:
```bash
cargo run
```

## Configuration

The service can be configured through multiple methods:

### Environment Variables
- `METRICS_COLLECTOR_BATCH_SIZE`: Number of pods to process in batch (default: 50)
- `METRICS_COLLECTOR_COLLECT_INTERVAL_SECS`: Metrics collection interval in seconds (default: 60)
- `METRICS_COLLECTOR_NAMESPACE`: Kubernetes namespace to monitor (default: "default")

### Using .env File
Create a `.env` file in the project root with the same variables as above:
```env
METRICS_COLLECTOR_BATCH_SIZE=10
METRICS_COLLECTOR_COLLECT_INTERVAL_SECS=30
METRICS_COLLECTOR_NAMESPACE=default
```

### Kubernetes ConfigMap
When deployed to Kubernetes, configure using ConfigMap:
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: metrics-collector-config
data:
  METRICS_COLLECTOR_BATCH_SIZE: "10"
  METRICS_COLLECTOR_COLLECT_INTERVAL_SECS: "30"
  METRICS_COLLECTOR_NAMESPACE: "default"
```

## Output Format

The service outputs JSON-formatted metrics data:
```json
{
  "timestamp": 1735819470,
  "pod_name": "example-pod",
  "namespace": "default",
  "total_cpu_usage": 0.951,
  "total_cpu_usage_millicores": "951m",
  "total_memory_usage": 83689472.0,
  "total_memory_usage_formatted": "78Mi",
  "containers": [
    {
      "container_name": "main",
      "cpu_usage": 0.339,
      "cpu_usage_millicores": "339m",
      "memory_usage": 83689472.0,
      "memory_usage_formatted": "78Mi",
      "volumes": [...]
    }
  ]
}
```

## Development

### Running Tests
```bash
cargo test
```

### Local Development Setup
1. Start Minikube
2. Enable metrics-server
3. Deploy test workloads
4. Create .env file
5. Run the service

### Project Structure
```
src/
├── main.rs               # Application entry point
├── config.rs             # Configuration management
├── metrics/
│   ├── mod.rs            # Metrics module
│   ├── collector.rs      # Metrics collection
│   ├── types.rs          # Data structures
│   └── parsers.rs        # Value parsing
└── kubernetes/
    ├── mod.rs            # Kubernetes module
    ├── client.rs         # K8s client wrapper
    └── errors.rs         # Error handling
```