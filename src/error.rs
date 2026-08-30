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
