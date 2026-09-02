use std::fmt;

/// Errors that can occur during OpenTelemetry initialisation or export.
#[derive(Debug)]
pub enum OtelError {
    /// The requested feature is not enabled.
    FeatureRequired(&'static str),
    /// An OTLP transport error.
    OtlpTransport(String),
    /// An exporter error from the SDK.
    Export(String),
}

impl fmt::Display for OtelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FeatureRequired(name) => write!(f, "feature `{name}` is required but not enabled"),
            Self::OtlpTransport(msg) => write!(f, "OTLP transport error: {msg}"),
            Self::Export(msg) => write!(f, "exporter error: {msg}"),
        }
    }
}

impl std::error::Error for OtelError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_required_display() {
        let err = OtelError::FeatureRequired("otlp");
        assert_eq!(
            err.to_string(),
            "feature `otlp` is required but not enabled"
        );
    }

    #[test]
    fn otlp_transport_display() {
        let err = OtelError::OtlpTransport("connection refused".into());
        assert_eq!(err.to_string(), "OTLP transport error: connection refused");
    }

    #[test]
    fn export_display() {
        let err = OtelError::Export("timeout".into());
        assert_eq!(err.to_string(), "exporter error: timeout");
    }

    #[test]
    fn error_is_std_error() {
        let err: Box<dyn std::error::Error> =
            Box::new(OtelError::FeatureRequired("stdout"));
        assert!(err.to_string().contains("stdout"));
    }

    #[test]
    fn debug_format() {
        let err = OtelError::FeatureRequired("otlp");
        let debug = format!("{:?}", err);
        assert!(debug.contains("FeatureRequired"));
        assert!(debug.contains("otlp"));
    }

    #[test]
    fn feature_required_various_names() {
        for name in &["otlp", "stdout", "prometheus", "grpc"] {
            let err = OtelError::FeatureRequired(name);
            assert!(err.to_string().contains(name));
        }
    }
}
