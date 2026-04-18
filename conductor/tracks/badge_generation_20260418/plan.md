# Implementation Plan - Badge Generation

## Phase 1: Client Helpers
- [ ] Task: Add `get_badge_url` and `get_uptime_badge_url` helpers to `GatusClient`.
- [ ] Task: Add support for fetching raw SVG content (optional).

## Phase 2: tool Integration
- [ ] Task: Add `get-badge` action to `get_metrics` tool.
- [ ] Task: Update `format_endpoint_status` to include a badge link by default.
- [ ] Task: Add integration tests.
