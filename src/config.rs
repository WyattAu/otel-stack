use crate::ExporterConfig;

/// Configuration for the OpenTelemetry stack.
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// Name of the service for resource identification.
    pub service_name: String,
    /// Service version.
    pub version: Option<String>,
    /// OTLP endpoint URL (e.g. `http://localhost:4317`).
    pub endpoint: Option<String>,
    /// Trace sample rate (0.0–1.0).
    pub sample_rate: f64,
    /// Exporter backend to use.
    pub exporter: ExporterConfig,
}

impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            service_name: "unknown-service".into(),
            version: None,
            endpoint: None,
            sample_rate: 1.0,
            exporter: ExporterConfig::Otlp,
        }
    }
}

impl OtelConfig {
    /// Start building a config with the given service name.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self::default().service_name(service_name)
    }

    /// Set the service name.
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.service_name = name.into();
        self
    }

    /// Set the service version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the OTLP endpoint.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Set the sample rate (0.0–1.0).
    pub fn sample_rate(mut self, rate: f64) -> Self {
        self.sample_rate = rate.clamp(0.0, 1.0);
        self
    }

    /// Set the exporter.
    pub fn exporter(mut self, exporter: ExporterConfig) -> Self {
        self.exporter = exporter;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = OtelConfig::default();
        assert_eq!(config.service_name, "unknown-service");
        assert!(config.version.is_none());
        assert!(config.endpoint.is_none());
        assert_eq!(config.sample_rate, 1.0);
        assert!(matches!(config.exporter, ExporterConfig::Otlp));
    }

    #[test]
    fn new_sets_service_name() {
        let config = OtelConfig::new("my-service");
        assert_eq!(config.service_name, "my-service");
    }

    #[test]
    fn builder_service_name() {
        let config = OtelConfig::default().service_name("api-server");
        assert_eq!(config.service_name, "api-server");
    }

    #[test]
    fn builder_version() {
        let config = OtelConfig::default().version("1.2.3");
        assert_eq!(config.version, Some("1.2.3".to_string()));
    }

    #[test]
    fn builder_endpoint() {
        let config = OtelConfig::default().endpoint("http://localhost:4317");
        assert_eq!(config.endpoint, Some("http://localhost:4317".to_string()));
    }

    #[test]
    fn builder_sample_rate_clamped() {
        let config = OtelConfig::default().sample_rate(2.0);
        assert_eq!(config.sample_rate, 1.0);

        let config = OtelConfig::default().sample_rate(-1.0);
        assert_eq!(config.sample_rate, 0.0);

        let config = OtelConfig::default().sample_rate(0.5);
        assert_eq!(config.sample_rate, 0.5);
    }

    #[test]
    fn builder_exporter() {
        let config = OtelConfig::default().exporter(ExporterConfig::Stdout);
        assert!(matches!(config.exporter, ExporterConfig::Stdout));
    }

    #[test]
    fn builder_chaining() {
        let config = OtelConfig::new("svc")
            .version("1.0")
            .endpoint("http://localhost:4317")
            .sample_rate(0.5)
            .exporter(ExporterConfig::Stdout);

        assert_eq!(config.service_name, "svc");
        assert_eq!(config.version, Some("1.0".to_string()));
        assert_eq!(config.endpoint, Some("http://localhost:4317".to_string()));
        assert_eq!(config.sample_rate, 0.5);
        assert!(matches!(config.exporter, ExporterConfig::Stdout));
    }

    #[test]
    fn clone_produces_equal_config() {
        let config = OtelConfig::new("svc").version("1.0");
        let cloned = config.clone();
        assert_eq!(config.service_name, cloned.service_name);
        assert_eq!(config.version, cloned.version);
    }

    #[test]
    fn debug_format() {
        let config = OtelConfig::new("svc");
        let debug = format!("{:?}", config);
        assert!(debug.contains("OtelConfig"));
        assert!(debug.contains("svc"));
    }
}
