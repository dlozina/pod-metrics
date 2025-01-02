use thiserror::Error;
use std::time::Duration;

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Kubernetes client error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("Failed to parse metric value: {0}")]
    ParseError(String),

    #[error("Metrics API timeout after {0:?}")]
    TimeoutError(Duration),

    #[error("Rate limit exceeded, retry after {0:?}")]
    RateLimitError(Duration),

    #[error("Pod {0} not found in namespace {1}")]
    PodNotFound(String, String),

    #[error("Metrics not available for pod {0} in namespace {1}")]
    MetricsNotAvailable(String, String),

    #[error("Invalid metric format: {0}")]
    InvalidMetricFormat(String),

    #[error("Metrics server not available: {0}")]
    MetricsServerError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl MetricsError {
    pub fn is_retriable(&self) -> bool {
        matches!(
            self,
            MetricsError::TimeoutError(_) |
            MetricsError::RateLimitError(_) |
            MetricsError::NetworkError(_) |
            MetricsError::MetricsServerError(_)
        )
    }

    pub fn retry_delay(&self) -> Duration {
        match self {
            MetricsError::RateLimitError(delay) => *delay,
            MetricsError::TimeoutError(_) => Duration::from_secs(5),
            MetricsError::NetworkError(_) => Duration::from_secs(1),
            MetricsError::MetricsServerError(_) => Duration::from_secs(10),
            _ => Duration::from_secs(1),
        }
    }

    pub fn should_alert(&self) -> bool {
        matches!(
            self,
            MetricsError::AuthorizationError(_) |
            MetricsError::MetricsServerError(_) |
            MetricsError::InternalError(_)
        )
    }
}

// Helper functions for common error cases
pub fn parse_error(value: impl Into<String>) -> MetricsError {
    MetricsError::ParseError(value.into())
}

pub fn timeout_error(duration: Duration) -> MetricsError {
    MetricsError::TimeoutError(duration)
}

pub fn rate_limit_error(retry_after: Duration) -> MetricsError {
    MetricsError::RateLimitError(retry_after)
}

pub fn pod_not_found(pod_name: impl Into<String>, namespace: impl Into<String>) -> MetricsError {
    MetricsError::PodNotFound(pod_name.into(), namespace.into())
}

// Extension trait for Result with metrics-specific helpers
pub trait MetricsResultExt<T> {
    fn on_metrics_error<F>(self, op: F) -> Result<T, MetricsError>
    where
        F: FnOnce() -> String;
}

impl<T> MetricsResultExt<T> for Result<T, MetricsError> {
    fn on_metrics_error<F>(self, op: F) -> Result<T, MetricsError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let context = op();
            MetricsError::InternalError(format!("{} - {}", context, e))
        })
    }
}

// Type alias for our Result type
pub type MetricsResult<T> = Result<T, MetricsError>;