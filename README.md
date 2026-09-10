# otel-stack

[![docs.rs](https://docs.rs/otel-stack/badge.svg)](https://docs.rs/otel-stack)
[![crates.io](https://img.shields.io/crates/v/otel-stack.svg)](https://crates.io/crates/otel-stack)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

OpenTelemetry integration for Rust — unified tracing, metrics, and OTLP export with version-pinned dependencies.

## Purpose

`otel-stack` provides a single, opinionated entry-point for adding OpenTelemetry instrumentation to Rust services. It pins every transitive dependency (tracing, opentelemetry-*, etc.) so that multiple crates in a workspace or across repos always share the **same** OTel versions — eliminating version-skew build failures and runtime panics.

## Features

| Feature    | Description |
|------------|-------------|
| `otlp` (default) | Export traces via OTLP/gRPC |
| `stdout`          | Print spans to stdout (dev/test) |
| `prometheus`      | Expose a Prometheus metrics endpoint |

## Usage

```rust,ignore
use otel_stack::{OtelStack, OtelConfig, ExporterConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = OtelStack::init(
        OtelConfig::new("my-service")
            .version(env!("CARGO_PKG_VERSION"))
            .sample_rate(0.1)
            .exporter(ExporterConfig::Otlp),
    )?;

    tracing::info!("service started");
    Ok(())
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
