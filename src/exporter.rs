//! Exporter backend selection.
//!
//! This is a compatibility facade: the variants describe intent, and all
//! initialization logic lives in [`otelkit`]. See [`ExporterConfig`] for the
//! mapping to [`otelkit::Exporter`].

/// Available exporter backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExporterConfig {
    /// Export via OTLP (requires the `otlp` feature).
    #[default]
    Otlp,
    /// Export to stdout (requires the `stdout` feature).
    Stdout,
    /// Expose Prometheus metrics (requires the `prometheus` feature).
    Prometheus,
}

impl From<ExporterConfig> for otelkit::Exporter {
    fn from(cfg: ExporterConfig) -> Self {
        match cfg {
            ExporterConfig::Otlp => otelkit::Exporter::Otlp,
            ExporterConfig::Stdout => otelkit::Exporter::Stdout,
            ExporterConfig::Prometheus => otelkit::Exporter::Prometheus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_format_otlp() {
        let config = ExporterConfig::Otlp;
        let debug = format!("{config:?}");
        assert_eq!(debug, "Otlp");
    }

    #[test]
    fn debug_format_stdout() {
        let config = ExporterConfig::Stdout;
        let debug = format!("{config:?}");
        assert_eq!(debug, "Stdout");
    }

    #[test]
    fn debug_format_prometheus() {
        let config = ExporterConfig::Prometheus;
        let debug = format!("{config:?}");
        assert_eq!(debug, "Prometheus");
    }

    #[test]
    fn default_is_otlp() {
        assert_eq!(ExporterConfig::default(), ExporterConfig::Otlp);
    }

    #[test]
    fn clone_produces_equal_config() {
        let configs = vec![
            ExporterConfig::Otlp,
            ExporterConfig::Stdout,
            ExporterConfig::Prometheus,
        ];
        for config in configs {
            let cloned = config;
            assert_eq!(format!("{config:?}"), format!("{cloned:?}"));
        }
    }

    #[test]
    fn converts_to_otelkit_exporter() {
        assert_eq!(
            otelkit::Exporter::from(ExporterConfig::Otlp),
            otelkit::Exporter::Otlp
        );
        assert_eq!(
            otelkit::Exporter::from(ExporterConfig::Stdout),
            otelkit::Exporter::Stdout
        );
        assert_eq!(
            otelkit::Exporter::from(ExporterConfig::Prometheus),
            otelkit::Exporter::Prometheus
        );
    }
}
