# Functional Specification - Token Usage Optimization via Tool Grouping

## Overview
To improve the efficiency of the Gatus MCP server and reduce the token consumption of LLMs interacting with it, the current set of granular tools will be grouped into logical categories. This reduces the number of tool definitions the LLM must process during discovery and simplifies the interaction model.

## Current Tool Set (Granular)
1. `list_services`: List all services (summary).
2. `get_endpoint_statuses`: Get detailed statuses (raw JSON).
3. `get_service_status`: Get status for a specific service (formatted).
4. `get_service_history`: Get history for a specific service (raw JSON).

## Proposed Grouped Tool Set
### 1. `manage_services`
- **Description:** Manage and list Gatus monitored services.
- **Actions:**
    - `list`: Returns a compact summary of all services and their current health.
    - `status`: Returns detailed current status for all endpoints.
- **Arguments:**
    - `action`: (Required) "list" or "status".

### 2. `get_service_info`
- **Description:** Retrieve detailed information or history for a specific service.
- **Actions:**
    - `details`: Returns current status and latest result for the specified service.
    - `history`: Returns recent health check results for the specified service.
- **Arguments:**
    - `service`: (Required) Name of the service.
    - `action`: (Required) "details" or "history".
    - `limit`: (Optional) Maximum number of results for the "history" action (default: 10).

## Technical Requirements
- **JSON-RPC Dispatching:** Update `handle_call_tool` in `src/mcp.rs` to handle the new grouped tool names and internal action dispatching.
- **Schema Definitions:** Update `get_tool_definitions` to reflect the new tool structures and required arguments.
- **Backward Compatibility:** For this specific task, we will replace the old tools with the new ones to strictly optimize token usage.
- **Response Formatting:** Ensure all tool outputs are consistently formatted (preferring Markdown for "list", "status", and "details").

## Success Criteria
- The number of tools listed by `tools/list` is reduced from 4 to 2.
- Each grouped tool correctly dispatches to the requested action.
- LLM can successfully perform all previous tasks using the new grouped tools.
- Token usage for tool discovery is significantly reduced.
