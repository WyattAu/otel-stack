#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! OpenTelemetry integration for Rust — unified tracing, metrics, and OTLP
//! export.
//!
//! This crate is a compatibility facade over [`otelkit`]: configuration
//! types ([`OtelConfig`], [`ExporterConfig`]) and the RAII lifecycle type
//! ([`OtelStack`]) are preserved, while all initialization logic delegates
//! to `otelkit`. New code should depend on `otelkit` directly.

mod config;
mod error;
mod exporter;

pub use config::OtelConfig;
pub use error::OtelError;
pub use exporter::ExporterConfig;

/// Re-exported for migration convenience: the underlying implementation.
pub use otelkit::TelemetryGuard;

/// RAII guard that shuts down the OpenTelemetry pipeline on drop.
///
/// Delegates lifecycle management to [`otelkit::TelemetryGuard`].
pub struct OtelStack {
    guard: Option<otelkit::TelemetryGuard>,
}

impl OtelStack {
    /// Initialize the telemetry stack from the given configuration.
    pub fn init(config: OtelConfig) -> Result<Self, OtelError> {
        let guard = otelkit::init(config.into_telemetry_config())?;
        Ok(OtelStack { guard: Some(guard) })
    }
}

impl Drop for OtelStack {
    fn drop(&mut self) {
        // Dropping the inner guard runs its shutdown sequence.
        self.guard.take();
    }
}
