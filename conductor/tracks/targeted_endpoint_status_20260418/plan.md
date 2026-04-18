# Implementation Plan - Targeted Endpoint Status

## Phase 1: Client Optimization [checkpoint: 078fe71]
- [x] Task: Implement `get_endpoint_statuses(key)` in `GatusClient`. [09473f1]
- [x] Task: Add unit tests with Wiremock. [5e521e5]

## Phase 2: tool Integration
- [x] Task: Update `get_metrics` (action: `service-history`) to use the targeted client method. [3edf21a]
- [ ] Task: Verify that token optimizations (stripping headers/truncating body) still apply.
- [ ] Task: Benchmark/Verify efficiency gains.
