# Implementation Plan: Detailed Certificate Audit

## Phase 1: Client Expansion (`src/client.rs`) [checkpoint: ba592d6]
- [x] Implement `get_certificate_audit` method. 5f7d577

## Phase 2: Formatting Enhancements (`src/fmt.rs`) [checkpoint: 41323fe]
- [x] Implement required formatting for new tool outputs. 1b1cb06

## Phase 3: MCP Handler Integration (`src/mcp.rs`)
- [ ] Update `get_tool_definitions` to include new tools and parameters.
- [ ] Update MCP handlers to route requests to new client methods.

## Phase 4: Verification
- [ ] Add integration tests in `tests/` for each new tool.
- [ ] Verify token efficiency of new payloads.
- [ ] Update documentation.
