# Requirements — otel-stack

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement.

Scope note: `otel-stack` is a compatibility facade over `otelkit` 2.x —
builder-style `OtelConfig` (service name, version, endpoint, sample rate,
exporter selection) and `OtelStack::init` that installs the telemetry
pipeline (OTLP, stdout, or Prometheus export).

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-OS-001 | `OtelConfig::new(service_name)` creates a config with OTLP as the default exporter | MUST |
| REQ-OS-002 | Builder methods (`service_name`, `version`, `endpoint`, `sample_rate`, `exporter`) chain and each stores its value observably | MUST |
| REQ-OS-003 | `sample_rate` values outside `[0.0, 1.0]` are clamped into the range | MUST |
| REQ-OS-004 | `ExporterConfig` maps to the corresponding `otelkit` exporter: OTLP (default), stdout, Prometheus; feature-gated backends report the missing feature by name | MUST |
| REQ-OS-005 | `OtelConfig::into_telemetry_config` converts the facade config into an `otelkit::TelemetryConfig` preserving all fields | MUST |
| REQ-OS-006 | `OtelStack::init` returns `Result<_, OtelError>`; SDK and provider failures surface as typed errors with sources preserved | MUST |
| REQ-OS-007 | `OtelConfig` is `Clone + PartialEq + Debug`, and cloning produces an equal config | SHOULD |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-OS-100 | No panic path in configuration handling: unknown exporter names, feature-disabled backends, and invalid configs produce `OtelError` values, never panics | MUST |
| REQ-OS-101 | Error taxonomy is `std::error::Error`-conformant; feature-required errors name the missing feature so misconfiguration is diagnosable | MUST |
| REQ-OS-102 | `Debug` output contains only configuration fields (service name, version, endpoint, exporter, sample rate); the facade models no secret material | SHOULD |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-OS-200 | All error variants render useful `Display` strings, including feature-required and telemetry-wrapped errors | SHOULD |
| REQ-OS-201 | OTLP transport selection is visible in `Debug` output (`otlp_transport_display`) | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-OS-001 | `default_config`, `default_is_otlp` (`src/lib.rs` tests) | unit |
| REQ-OS-002 | `builder_chaining`, `builder_service_name`, `builder_version`, `builder_endpoint`, `builder_exporter`, `builder_sample_rate_clamped` | unit |
| REQ-OS-003 | `builder_sample_rate_clamped` | unit |
| REQ-OS-004 | `builder_exporter`, `converts_to_otelkit_exporter`, `export_display`, `feature_required_display`, `feature_required_various_names` | unit |
| REQ-OS-005 | `into_telemetry_config_defaults`, `into_telemetry_config_maps_all_fields` | unit |
| REQ-OS-006 | `from_telemetry_error_config`, `from_telemetry_error_otlp`, `invalid_config_display`, `new_sets_service_name` | unit |
| REQ-OS-007 | `clone_produces_equal_config`, `debug_format` | unit |
| REQ-OS-100 | `feature_required_various_names`, `invalid_config_display` | unit |
| REQ-OS-101 | `error_is_std_error`, `feature_required_display` | unit |
| REQ-OS-102 | `debug_format`, `debug_format_otlp`, `debug_format_prometheus`, `debug_format_stdout`, `otlp_transport_display` | unit |
| REQ-OS-200 | `error_is_std_error`, `feature_required_display`, `invalid_config_display` | unit |
| REQ-OS-201 | `otlp_transport_display`, `debug_format_otlp` | unit |

## Test Count

- 27 `#[test]` functions in the unit suite.
- All-features suite passes with 0 failures; no-default-features suite passes.
