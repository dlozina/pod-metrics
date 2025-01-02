use tracing::{info, warn};
use crate::kubernetes::errors::{MetricsError, MetricsResult};

pub fn parse_cpu(cpu_str: &str) -> MetricsResult<f64> {
    info!("Parsing CPU value: {}", cpu_str);
    
    if let Some(nano_str) = cpu_str.strip_suffix('n') {
        return nano_str
            .parse::<f64>()
            .map(|nanocores| nanocores / 1_000_000_000.0)
            .map_err(|_| MetricsError::ParseError(
                format!("Invalid nanocores CPU value: {}", cpu_str)
            ));
    }
    
    if let Some(milli_str) = cpu_str.strip_suffix('m') {
        return milli_str
            .parse::<f64>()
            .map(|millicores| millicores / 1000.0)
            .map_err(|_| MetricsError::ParseError(
                format!("Invalid millicores CPU value: {}", cpu_str)
            ));
    }

    cpu_str
        .parse::<f64>()
        .map_err(|_| {
            warn!("Failed to parse CPU value: {}", cpu_str);
            MetricsError::ParseError(format!("Invalid CPU value: {}", cpu_str))
        })
}

pub fn parse_memory(memory_str: &str) -> MetricsResult<f64> {
    info!("Parsing Memory value: {}", memory_str);

    if let Some(mi_str) = memory_str.strip_suffix("Mi") {
        return mi_str
            .parse::<f64>()
            .map(|mi| {
                info!("Parsed Mi value: {} megabytes", mi);
                mi * 1024.0 * 1024.0
            })
            .map_err(|_| MetricsError::ParseError(
                format!("Invalid megabytes value: {}", memory_str)
            ));
    }

    if let Some(ki_str) = memory_str.strip_suffix("Ki") {
        return ki_str
            .parse::<f64>()
            .map(|ki| {
                info!("Parsed Ki value: {} kilobytes", ki);
                ki * 1024.0
            })
            .map_err(|_| MetricsError::ParseError(
                format!("Invalid kilobytes value: {}", memory_str)
            ));
    }

    if let Some(gi_str) = memory_str.strip_suffix("Gi") {
        return gi_str
            .parse::<f64>()
            .map(|gi| {
                info!("Parsed Gi value: {} gigabytes", gi);
                gi * 1024.0 * 1024.0 * 1024.0
            })
            .map_err(|_| MetricsError::ParseError(
                format!("Invalid gigabytes value: {}", memory_str)
            ));
    }

    memory_str
        .parse::<f64>()
        .map_err(|_| {
            warn!("Failed to parse Memory value: {}", memory_str);
            MetricsError::ParseError(format!("Invalid memory value: {}", memory_str))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_nanocores() {
        assert_eq!(parse_cpu("1000000000n").unwrap(), 1.0);
    }

    #[test]
    fn test_parse_cpu_millicores() {
        assert_eq!(parse_cpu("1000m").unwrap(), 1.0);
    }

    #[test]
    fn test_parse_memory_mi() {
        assert_eq!(parse_memory("1Mi").unwrap(), 1048576.0);
    }

    #[test]
    fn test_parse_memory_gi() {
        assert_eq!(parse_memory("1Gi").unwrap(), 1073741824.0);
    }

    #[test]
    fn test_invalid_cpu() {
        assert!(parse_cpu("invalid").is_err());
    }

    #[test]
    fn test_invalid_memory() {
        assert!(parse_memory("invalid").is_err());
    }
}