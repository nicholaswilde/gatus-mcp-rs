# Track Specification: Refactor Functions for Token Efficiency

## Overview
The goal of this track is to consolidate the existing Gatus MCP tools into a smaller, more dynamic set of functions. This refactoring aims to reduce the overall size of the tool discovery schema and optimize the data payloads returned to LLMs, thereby maximizing token efficiency and improving the overall effectiveness of AI-driven interactions.

## Functional Requirements
- **Tool Consolidation**: Consolidate existing granular tools (e.g., `get_endpoint_stats`, `get_uptime`, `get_alert_history`, `get_config`, `manage_services`) into a highly dynamic and parameterized toolset.
- **Dynamic Tool Definitions**: Utilize tool arguments (e.g., `action`, `type`, `target`) to handle diverse queries through a reduced number of tool definitions.
- **Payload Optimization**: Implement comprehensive thinning of response payloads to ensure only high-signal data is returned, minimizing token consumption in the conversation history.
- **Breaking Changes**: Implement an optimized, clean-break toolset without the constraint of backward compatibility for existing implementations.

## Non-Functional Requirements
- **Token Efficiency**: Achieve a significant reduction (targeted >30%) in the number of tokens required for tool discovery and typical data retrieval flows.
- **Performance**: Maintain or improve the low latency of tool execution.
- **Maintainability**: Ensure the consolidated tool dispatching logic is clean, modular, and easy to extend.

## Acceptance Criteria
- [ ] The number of tool definitions exposed to the LLM is significantly reduced.
- [ ] Consolidated tools correctly handle all functionality previously provided by the granular toolset.
- [ ] Tool responses are verified to be "thin" and high-signal, without redundant JSON metadata.
- [ ] Comprehensive test suite (unit and live) verifies the behavioral correctness of the new toolset.
- [ ] `README.md` and project documentation are updated to reflect the new tool schema.

## Out of Scope
- Implementing entirely new Gatus API client functionality (focus is on refactoring existing access patterns).
- Modifying the underlying SSE transport layer.
