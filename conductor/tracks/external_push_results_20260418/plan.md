# Implementation Plan - External Endpoint Push

## Phase 1: Client Support [checkpoint: 20ef6a6]
- [x] Task: Implement `push_endpoint_result` in `GatusClient`. [86d9d32]
- [x] Task: Add unit tests for the new client method. [20ef6a6]

## Phase 2: MCP Integration
- [ ] Task: Add `push-result` action to the mutative tool handling.
- [ ] Task: Update tool schema in `mcp.rs`.
- [ ] Task: Add integration tests (mocked and live).
