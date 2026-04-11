# Functional Specification - Stdio Status Output

## Overview
When running the Gatus MCP server in `stdio` mode, it currently communicates silently over JSON-RPC on `stdout`. This makes it difficult for agents (LLMs or other processes) to verify that the server has started correctly, what its configuration is, and if it's successfully communicating with Gatus.

## Requirements
- **Stderr Logging:** All status and diagnostic messages MUST be output to `stderr` to avoid breaking the MCP protocol on `stdout`.
- **Startup Information:** Upon starting in `stdio` mode, the server should log:
    - Its version.
    - The Gatus API URL it's configured to use.
    - A clear "Ready to receive MCP messages" message.
- **Activity Logging:** (Optional/Level-based) Log when a request is received and a response is sent (at `debug` level).
- **Error Reporting:** Log connectivity issues with Gatus or protocol errors to `stderr`.

## Technical Requirements
- Use the `tracing` crate (already in use) for all status output.
- Ensure `tracing-subscriber` is configured to output to `stderr` by default (usually it does).

## Success Criteria
- Running `gatus-mcp-rs stdio` outputs a "Ready" status to `stderr`.
- The output includes the Gatus API URL.
- The MCP protocol on `stdout` remains valid JSON-RPC.
