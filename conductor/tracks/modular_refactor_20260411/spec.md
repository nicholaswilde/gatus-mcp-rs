# Specification - Modular Refactor to Match Rescue-Groups-MCP Shape

## Overview
Refactor the Gatus MCP Server to adopt the modular architecture and dual-transport support (Stdio + SSE) found in the `rescue-groups-mcp` project. This will improve maintainability, testability, and client compatibility.

## Goals
- **Modularize Codebase:** Split logic into `mcp.rs`, `server.rs`, `client.rs`, `cli.rs`, and `fmt.rs`.
- **Dual Transport Support:** Implement both Stdio and SSE/HTTP transport modes.
- **Enhanced CLI:** Use Clap subcommands for `stdio`, `http`, and direct tool execution.
- **Improved Taskfile:** Add cross-compilation and coverage tasks.
- **Caching:** Integrate `moka` for Gatus API response caching.

## Target Structure
- `src/main.rs`: Entry point, initializes CLI and routes to transport.
- `src/cli.rs`: Command-line interface definition (Clap).
- `src/mcp.rs`: MCP protocol handler (JSON-RPC 2.0).
- `src/server.rs`: Stdio and HTTP/SSE server implementations.
- `src/client.rs`: Gatus API client logic.
- `src/fmt.rs`: Markdown formatting for tool responses.
- `src/settings.rs`: Configuration management.
