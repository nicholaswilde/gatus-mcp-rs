# Product Guidelines - Gatus MCP Server

## Technical Philosophy
- **Reliability First:** As a monitoring tool proxy, the server must be stable and handle Gatus API errors gracefully.
- **Minimal Overhead:** The server should be lightweight, ensuring that the "observer effect" on the monitored system is negligible.
- **Security by Design:** Sensitive information (API keys, endpoints) must be handled securely via environment variables or encrypted configuration.

## MCP Implementation Principles
- **Tool Clarity:** Tool names and descriptions must be clear and descriptive to allow LLMs to select the correct tool for a given task.
- **Strategic Tool Consolidation:** Prioritize a consolidated toolset (e.g., `manage_resources`, `get_metrics`) using dynamic `action` arguments to minimize the tool discovery schema and optimize the LLM's context window.
- **Standardized Workflows (MCP Prompts):** Use pre-defined prompts to guide LLMs through complex or multi-step tasks like troubleshooting an outage or summarizing daily health.
- **Direct Context Attachment (MCP Resources):** Expose static or slowly-changing data as resources to allow LLMs to read state without executing tool calls.
- **Schema Optimization:** Use strictly defined JSON schemas for tool arguments to minimize LLM hallucinations.
- **Thin Payloads:** Always prefer returning summarized or filtered data over raw Gatus API responses to preserve the LLM's context window.

## Code & Architecture Guidelines
- **Decoupled Design:** Maintain a clear separation between the transport (Axum/SSE), protocol (MCP), and domain (Gatus API) layers.
- **Asynchronous Execution:** All I/O operations must be non-blocking using Tokio.
- **Comprehensive Logging:** Use the `tracing` crate for structured logging, especially for tracking MCP session lifecycle and tool execution.
- **Type Safety:** Leverage Rust's strong type system to ensure data integrity across the server.

## Operational Standards
- **Standardized Configuration:** Use `config-rs` to merge settings from multiple sources (Toml, Env, CLI).
- **Graceful Shutdown:** Ensure the server handles termination signals properly, closing active SSE connections and cleaning up resources.
- **Health Checks:** The server should expose its own health endpoint to ensure it is running correctly within its deployment environment.

## Documentation Requirements
- **Clear Installation Guide:** Provide concise steps for building and running the server.
- **Tool Reference:** Document each MCP tool, its purpose, and its expected arguments.
- **Troubleshooting Section:** Include common issues and their resolutions.
