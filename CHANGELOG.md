# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [0.2.0] - 2026-09-08

### Changed

- **Restructured as a compatibility facade over `otelkit` 2.x**: all
  initialization logic (OTLP/stdout/Prometheus exporters, Sentry, guard
  lifecycle) now delegates to `otelkit`. The public surface
  (`OtelConfig`, `OtelStack`, `ExporterConfig`, `OtelError`) is preserved,
  plus a new `OtelError::InvalidConfig` variant and
  `OtelConfig::into_telemetry_config()`.
- `ExporterConfig` is now `Copy` + `Default` (defaults to `Otlp`).
- Requires `otelkit` 2.0.

## [0.1.0] - 2026-09-02

Initial release with standalone OTLP/stdout/Prometheus exporter scaffolding.
