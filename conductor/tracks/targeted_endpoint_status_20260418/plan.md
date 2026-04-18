# Implementation Plan - Targeted Endpoint Status

## Phase 1: Client Optimization
- [ ] Task: Implement `get_endpoint_statuses(key)` in `GatusClient`.
- [ ] Task: Add unit tests with Wiremock.

## Phase 2: tool Integration
- [ ] Task: Update `get_metrics` (action: `service-history`) to use the targeted client method.
- [ ] Task: Verify that token optimizations (stripping headers/truncating body) still apply.
- [ ] Task: Benchmark/Verify efficiency gains.
