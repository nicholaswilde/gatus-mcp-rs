# Implementation Plan - Deep-Dive Diagnostics

## Phase 1: Raw Data Retrieval
- [x] Task: Update `GatusClient` to return raw result objects. (7cbea9d)
- [x] Task: Add `get-raw-results` action to `get_metrics`. (783f5ee)
- [ ] Task: Add unit tests for raw data deserialization.

## Phase 2: Enhanced Configuration
- [ ] Task: Implement detailed condition status reporting in `format_endpoint_status`.
- [ ] Task: Update integration tests to verify detailed failure reporting.
