# Implementation Plan: Multi-Suite Support

## Phase 1: Client Expansion (`src/client.rs`) [checkpoint: f16f905]
- [x] Implement `list_suites` method. 502c849
- [x] Implement `get_suite_health` method. 502c849

## Phase 2: Formatting Enhancements (`src/fmt.rs`) [checkpoint: 8caf3d1]
- [x] Implement required formatting for new tool outputs. efa8e78

## Phase 3: MCP Handler Integration (`src/mcp.rs`) [checkpoint: 801b337]
- [x] Update `get_tool_definitions` to include new tools and parameters. 8fcaba6
- [x] Update MCP handlers to route requests to new client methods. 8fcaba6

## Phase 4: Verification
- [x] Add integration tests in `tests/` for each new tool. 4cbd1aa
- [x] Verify token efficiency of new payloads. 4cbd1aa
- [x] Update documentation. 4cbd1aa
