# Implementation Plan - External Endpoint Push

## Phase 1: Client Support
- [ ] Task: Implement `push_endpoint_result` in `GatusClient`.
- [ ] Task: Add unit tests for the new client method.

## Phase 2: MCP Integration
- [ ] Task: Add `push-result` action to the mutative tool handling.
- [ ] Task: Update tool schema in `mcp.rs`.
- [ ] Task: Add integration tests (mocked and live).
