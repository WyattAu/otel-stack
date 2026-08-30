#![forbid(unsafe_code)]

mod config;
mod error;
mod exporter;

pub use config::OtelConfig;
pub use error::OtelError;
pub use exporter::ExporterConfig;

use opentelemetry::global;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// RAII guard that shuts down the OpenTelemetry pipeline on drop.
pub struct OtelStack {
    provider: Option<SdkTracerProvider>,
}

impl OtelStack {
    pub fn init(config: OtelConfig) -> Result<Self, OtelError> {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

        let exporter = config.exporter.build()?;
        let provider = exporter.build_provider(&config.service_name);
        let tracer = provider.tracer(config.service_name.clone());

        let telemetry = OpenTelemetryLayer::new(tracer);

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_target(true))
            .with(telemetry)
            .init();

        global::set_tracer_provider(provider.clone());

        Ok(OtelStack {
            provider: Some(provider),
        })
    }
}

impl Drop for OtelStack {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            if let Err(e) = provider.shutdown() {
                eprintln!("OpenTelemetry shutdown error: {e}");
            }
        }
    }
}
