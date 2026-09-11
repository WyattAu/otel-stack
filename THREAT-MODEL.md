# Threat Model — otel-stack

Status: **v1.0** · Method: STRIDE over the public API surface
(`OtelStack`, `OtelConfig`, `ExporterConfig`, `OtelError`).

Trust boundaries: (1) operator-supplied configuration (service name,
endpoint, exporter selection, sample rate), (2) the telemetry pipeline
itself (spans/metrics flow to the configured exporter backend over the
network), (3) the `otelkit`/`opentelemetry` dependency stack that this
crate wraps.

As a thin compatibility facade, most security properties are inherited
from `otelkit` 2.x. This model covers what the facade itself adds:
configuration validation, exporter selection, and error surfacing.

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Correct telemetry routing | Spans silently dropped or sent to a wrong collector due to config coercion |
| A2 | Availability of the host application | `init` panicking on bad config, taking the app down at boot |
| A3 | Configuration fidelity | Silent clamping/coercion of endpoint or sample rate that operators cannot observe |
| A4 | Error diagnosability | OTEL SDK setup failures swallowed into generic errors, hiding misconfig |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Bad exporter name crashes the app at init | DoS | `ExporterConfig::exporter` | Unknown exporter names produce `OtelError::FeatureRequired`/typed errors instead of panics; feature-gated backends report the missing feature by name | `feature_required_display`, `feature_required_various_names`, `error_is_std_error` |
| T2 | Endpoint/hostile URI accepted unchecked and telemetry exfiltrated | Spoofing/Elevation | `OtelConfig::endpoint` | The endpoint is stored verbatim and surfaced via `Debug`; the actual transport security is delegated to the OTLP exporter builder in `otelkit` — the facade never rewrites or downgrades the scheme | `builder_endpoint`, `otlp_transport_display`, `debug_format_otlp` |
| T3 | Sample-rate coercion silently changes observability SLOs | Tampering | `OtelConfig::sample_rate` | Sample rates are clamped to `[0.0, 1.0]` and the clamped value is observable via `Debug`/accessors | `builder_sample_rate_clamped`, `debug_format` |
| T4 | Init failure at boot takes the process down | DoS | `OtelStack::init` | Initialization returns `Result<_, OtelError>`; SDK/provider errors map through `OtelError::Telemetry` with source preserved | `from_telemetry_error_config`, `from_telemetry_error_otlp`, `invalid_config_display` |
| T5 | Config drift between clones / Debug leaks | Information Disclosure | `OtelConfig` | Config is `Clone + Debug + PartialEq`; cloning produces an equal config and Debug contains only non-secret fields (service name, endpoint, exporter) — no credentials are modeled by this crate | `clone_produces_equal_config`, `debug_format`, `debug_format_prometheus`, `debug_format_stdout` |
| T6 | Exporter mis-selection sends production spans to stdout | Tampering | `ExporterConfig` | Exporter default is OTLP (`default_is_otlp`); every selection path is unit-tested to map to the intended `otelkit` exporter enum | `default_is_otlp`, `converts_to_otelkit_exporter`, `export_display` |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — endpoint authenticity is operator-trust.** The facade does
  not pin or validate the collector endpoint beyond type checks; a
  misconfigured endpoint sends telemetry (potentially containing
  identifying attributes) to an attacker-controlled collector.
  Mitigation is deployment-level (network policy, mTLS at collector).
- **OPEN-2 — no sample-rate persistence.** Clamping happens at config
  time only; callers mutating `sample_rate` after init will not affect a
  running pipeline. Documented, not enforced.

## Out of Scope

- Security of `otelkit`/OpenTelemetry SDK internals (covered by
  `otelkit`'s own threat model).
- Collector-side security (authentication, quota, PII scrubbing).
- Resource attribution spoofing (service name/version are operator
  inputs by design).

## Residual Risks

- A wrong-but-valid exporter choice (e.g. stdout in production) is a
  configuration error the crate cannot detect; it is observable via
  `debug_format_*` tests patterns and `OtelConfig` accessors.
