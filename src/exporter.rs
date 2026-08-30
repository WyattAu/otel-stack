use crate::OtelError;
use opentelemetry_sdk::trace::SdkTracerProvider;

/// Available exporter backends.
#[derive(Debug, Clone)]
pub enum ExporterConfig {
    /// Export via OTLP (requires the `otlp` feature).
    Otlp,
    /// Export to stdout (requires the `stdout` feature).
    Stdout,
    /// Expose Prometheus metrics (requires the `prometheus` feature).
    Prometheus,
}

impl ExporterConfig {
    pub(crate) fn build(&self) -> Result<Box<dyn TracerExporter>, OtelError> {
        match self {
            ExporterConfig::Otlp => {
                #[cfg(feature = "otlp")]
                {
                    Ok(Box::new(OtlpExporter))
                }
                #[cfg(not(feature = "otlp"))]
                {
                    Err(OtelError::FeatureRequired("otlp"))
                }
            }
            ExporterConfig::Stdout => {
                #[cfg(feature = "stdout")]
                {
                    Ok(Box::new(StdoutExporter))
                }
                #[cfg(not(feature = "stdout"))]
                {
                    Err(OtelError::FeatureRequired("stdout"))
                }
            }
            ExporterConfig::Prometheus => {
                #[cfg(feature = "prometheus")]
                {
                    Ok(Box::new(PrometheusExporter))
                }
                #[cfg(not(feature = "prometheus"))]
                {
                    Err(OtelError::FeatureRequired("prometheus"))
                }
            }
        }
    }
}

pub(crate) trait TracerExporter {
    fn build_provider(&self, service_name: &str) -> SdkTracerProvider;
}

#[cfg(feature = "otlp")]
struct OtlpExporter;

#[cfg(feature = "otlp")]
impl TracerExporter for OtlpExporter {
    fn build_provider(&self, _service_name: &str) -> SdkTracerProvider {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .build()
            .expect("failed to build OTLP exporter");
        SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .build()
    }
}

#[cfg(feature = "stdout")]
struct StdoutExporter;

#[cfg(feature = "stdout")]
impl TracerExporter for StdoutExporter {
    fn build_provider(&self, _service_name: &str) -> SdkTracerProvider {
        SdkTracerProvider::builder().build()
    }
}

#[cfg(feature = "prometheus")]
struct PrometheusExporter;

#[cfg(feature = "prometheus")]
impl TracerExporter for PrometheusExporter {
    fn build_provider(&self, _service_name: &str) -> SdkTracerProvider {
        SdkTracerProvider::default()
    }
}
