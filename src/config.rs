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
