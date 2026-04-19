# Implementation Plan - Deep-Dive Diagnostics

## Phase 1: Raw Data Retrieval [checkpoint: 3af3a0e]
- [x] Task: Update `GatusClient` to return raw result objects. (7cbea9d)
- [x] Task: Add `get-raw-results` action to `get_metrics`. (783f5ee)
- [x] Task: Add unit tests for raw data deserialization. (8ad25f8)

## Phase 2: Enhanced Configuration
- [x] Task: Implement detailed condition status reporting in `format_endpoint_status`. (fc6b99f)
- [x] Task: Update integration tests to verify detailed failure reporting. (c2d16f3)
