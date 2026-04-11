# Implementation Plan: Self-Hosted API Enhancements

## Phase 1: Client & Data Structures [x]
- [x] Define data structures for uptime and response time statistics in `src/client.rs`.
- [x] Implement `get_endpoint_uptimes` method in `GatusClient`.
- [x] Implement `get_endpoint_response_times` method in `GatusClient`.
- [x] Implement `get_instance_health` method in `GatusClient`.

## Phase 2: MCP Tool Integration [x]
- [x] Add `get_endpoint_stats` tool definition to `src/mcp.rs`.
- [x] Add `get_instance_health` tool definition to `src/mcp.rs`.
- [x] Implement `handle_get_endpoint_stats_tool` in `McpHandler`.
- [x] Implement `handle_get_instance_health_tool` in `McpHandler`.

## Phase 3: Testing & Verification [x]
- [x] Add unit tests for new `GatusClient` methods in `tests/test_client_self_hosted.rs`.
- [x] Add live tests for the new MCP tools in `tests/test_mcp_self_hosted.rs`.
- [x] Verify functionality with `task test`.
