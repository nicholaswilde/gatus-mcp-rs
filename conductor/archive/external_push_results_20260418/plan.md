# Implementation Plan - External Endpoint Push

## Phase 1: Client Support [checkpoint: 20ef6a6]
- [x] Task: Implement `push_endpoint_result` in `GatusClient`. [86d9d32]
- [x] Task: Add unit tests for the new client method. [20ef6a6]

## Phase 2: MCP Integration [checkpoint: a3bda1f]
- [x] Task: Add `push-result` action to the mutative tool handling. [a3bda1f]
- [x] Task: Update tool schema in `mcp.rs`. [a3bda1f]
- [x] Task: Add integration tests (mocked and live). [a3bda1f]
