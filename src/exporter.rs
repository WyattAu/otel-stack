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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_format_otlp() {
        let config = ExporterConfig::Otlp;
        let debug = format!("{:?}", config);
        assert_eq!(debug, "Otlp");
    }

    #[test]
    fn debug_format_stdout() {
        let config = ExporterConfig::Stdout;
        let debug = format!("{:?}", config);
        assert_eq!(debug, "Stdout");
    }

    #[test]
    fn debug_format_prometheus() {
        let config = ExporterConfig::Prometheus;
        let debug = format!("{:?}", config);
        assert_eq!(debug, "Prometheus");
    }

    #[test]
    fn clone_produces_equal_config() {
        let configs = vec![
            ExporterConfig::Otlp,
            ExporterConfig::Stdout,
            ExporterConfig::Prometheus,
        ];
        for config in configs {
            let cloned = config.clone();
            assert_eq!(format!("{:?}", config), format!("{:?}", cloned));
        }
    }

    #[test]
    fn build_otlp_with_feature() {
        // With the default "otlp" feature, this should succeed
        let result = ExporterConfig::Otlp.build();
        assert!(result.is_ok());
    }

    #[test]
    fn build_stdout_requires_feature() {
        let result = ExporterConfig::Stdout.build();
        #[cfg(not(feature = "stdout"))]
        {
            assert!(result.is_err());
            match result {
                Err(OtelError::FeatureRequired(name)) => assert_eq!(name, "stdout"),
                _ => panic!("expected FeatureRequired error"),
            }
        }
    }

    #[test]
    fn build_prometheus_requires_feature() {
        let result = ExporterConfig::Prometheus.build();
        #[cfg(not(feature = "prometheus"))]
        {
            assert!(result.is_err());
            match result {
                Err(OtelError::FeatureRequired(name)) => assert_eq!(name, "prometheus"),
                _ => panic!("expected FeatureRequired error"),
            }
        }
    }

    #[test]
    fn missing_feature_error_message() {
        #[cfg(not(feature = "stdout"))]
        {
            let result = ExporterConfig::Stdout.build();
            assert!(result.is_err());
            let err = match result {
                Err(e) => e,
                Ok(_) => panic!("expected error"),
            };
            assert!(err.to_string().contains("stdout"));
        }
        #[cfg(not(feature = "prometheus"))]
        {
            let result = ExporterConfig::Prometheus.build();
            assert!(result.is_err());
            let err = match result {
                Err(e) => e,
                Ok(_) => panic!("expected error"),
            };
            assert!(err.to_string().contains("prometheus"));
        }
    }
}
