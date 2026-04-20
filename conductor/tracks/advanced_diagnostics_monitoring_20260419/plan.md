# Implementation Plan: Advanced Diagnostics and Monitoring Enhancements

## Phase 1: Client Expansion (`src/client.rs`)
- [x] Implement `get_expiring_certificates` method. (9760f3c)
- [x] Update `list_services` to accept an optional status filter. (37616d8)
- [x] Implement `get_failure_summary` logic in `GatusClient` or `EndpointStatus`. (a7808e8)
- [x] Implement `compare_performance` method calculating hour-vs-week latency delta. (1fdd157)
- [x] Implement `get_group_stats` to aggregate endpoint states by group. (a12daf2)
- [x] Implement `get_notification_events` to correlate alerts with failures. (1668212)
- [x] Implement `set_maintenance` (Dropped: Not supported by self-hosted Gatus API).
- [x] Implement `get_flapping_services` by parsing metrics. (fe0acef)

## Phase 2: Formatting Enhancements (`src/fmt.rs`)
- [ ] Add `format_expiring_certificates` table formatter.
- [ ] Add `format_failure_summary` markdown formatter.
- [ ] Add `format_group_stats` summary formatter.
- [ ] Add `format_alert_correlation` timeline formatter.

## Phase 3: MCP Handler Integration (`src/mcp.rs`)
- [ ] Update `get_tool_definitions` to include new tools and parameters.
- [ ] Update `manage_resources` handler for `list-expiring-certificates` and `set-maintenance`.
- [ ] Update `get_metrics` handler for `failure-summary`, `performance-comparison`, `group-stats`, `alert-correlation`, and `flapping-services`.
- [ ] Update `list-services` tool definition to include `status` parameter.

## Phase 4: Verification
- [ ] Add integration tests in `tests/` for each new tool.
- [ ] Verify token efficiency of new payloads.
- [ ] Update documentation.
