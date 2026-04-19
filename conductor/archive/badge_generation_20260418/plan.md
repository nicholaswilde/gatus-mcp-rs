# Implementation Plan - Badge Generation

## Phase 1: Client Helpers
- [x] Task: Add `get_badge_url` and `get_uptime_badge_url` helpers to `GatusClient`. [9352b54]
- [x] Task: Add support for fetching raw SVG content (optional). (Skipped)

## Phase 2: tool Integration
- [x] Task: Add `get-badge` action to `get_metrics` tool. [bd095d5]
- [x] Task: Update `format_endpoint_status` to include a badge link by default. [d6cc690]
- [x] Task: Add integration tests. [05e3c12]
