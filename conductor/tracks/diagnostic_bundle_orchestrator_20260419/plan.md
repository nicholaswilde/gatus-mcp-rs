# Implementation Plan: Diagnostic Bundle Orchestrator

## Phase 1: Client Expansion (`src/client.rs`) [checkpoint: 18c11c6]
- [x] Implement `get_diagnostic_bundle` method. 6da9295

## Phase 2: Formatting Enhancements (`src/fmt.rs`)
- [ ] Implement required formatting for new tool outputs.

## Phase 3: MCP Handler Integration (`src/mcp.rs`)
- [ ] Update `get_tool_definitions` to include new tools and parameters.
- [ ] Update MCP handlers to route requests to new client methods.

## Phase 4: Verification
- [ ] Add integration tests in `tests/` for each new tool.
- [ ] Verify token efficiency of new payloads.
- [ ] Update documentation.
