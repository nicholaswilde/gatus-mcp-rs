# Implementation Plan - Token Usage Optimization via Tool Grouping

## Phase 1: Planning & Setup
- [x] Task: Review current `src/mcp.rs` tool definitions.
- [x] Task: Update `conductor/product-guidelines.md` with tool grouping principles.

## Phase 2: Refactoring Tool Definitions
- [x] Task: Update `McpHandler::get_tool_definitions` in `src/mcp.rs`.
    - [x] Define `manage_services` tool.
    - [x] Define `get_service_info` tool.
    - [x] Remove individual granular tool definitions.

## Phase 3: Implementing Dispatching Logic
- [x] Task: Refactor `McpHandler::handle_call_tool` in `src/mcp.rs`.
    - [x] Implement dispatching for `manage_services` (actions: `list`, `status`).
    - [x] Implement dispatching for `get_service_info` (actions: `details`, `history`).
    - [x] Refactor existing handler methods to accommodate the new action-based structure.

## Phase 4: Verification & Documentation
- [x] Task: Update unit tests to reflect the new tool structure.
    - [x] Update `tests/test_mcp_list_services.rs`.
    - [x] Update `tests/test_mcp_get_service_status.rs`.
    - [x] Update `tests/test_mcp_get_service_history.rs`.
- [x] Task: Run `task test:ci` to ensure project integrity.
- [ ] Task: Run `task test:live` (optional) to confirm against a real instance.
- [ ] Task: Run `task test:live` (optional) to confirm against a real instance.
