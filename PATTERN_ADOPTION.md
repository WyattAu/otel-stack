# Pattern adoption

| Pattern | Status | Evidence |
|---|---|---|
| T13 | partial | src/lib.rs:7 (compatibility facade: config types and RAII OtelStack shell), src/exporter.rs:20 (conversion delegates initialization to otelkit) |
| P114 | n/a | no HTTP or JWT surface exists in a telemetry facade |
