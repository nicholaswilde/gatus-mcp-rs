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
- [x] Add `format_expiring_certificates` table formatter. (76c3679)
- [x] Add `format_failure_summary` markdown formatter. (76c3679)
- [x] Add `format_group_stats` summary formatter. (76c3679)
- [x] Add `format_alert_correlation` timeline formatter. (76c3679)

## Phase 3: MCP Handler Integration (`src/mcp.rs`)
- [x] Update `get_tool_definitions` to include new tools and parameters. (7875a35)
- [x] Update `manage_resources` handler for `list-expiring-certificates` and `set-maintenance`. (7875a35)
- [x] Update `get_metrics` handler for `failure-summary`, `performance-comparison`, `group-stats`, `alert-correlation`, and `flapping-services`. (7875a35)
- [x] Update `list-services` tool definition to include `status` parameter. (7875a35)

## Phase 4: Verification
- [x] Add integration tests in `tests/` for each new tool. (8c05c37)
- [x] Verify token efficiency of new payloads. (b4fdb24)
- [x] Update documentation. (fed8a35)
