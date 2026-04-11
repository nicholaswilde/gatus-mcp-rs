# Specification - Build core Gatus MCP server functionality

## Overview
This track focuses on building the foundational Gatus MCP server. It involves setting up the Rust project, implementing the SSE transport layer using Axum, handling the MCP JSON-RPC protocol, and integrating the initial Gatus API tools for service health monitoring.

## Goals
- Scaffolding a robust Rust project with the "Gatus Stack".
- Implementing a thread-safe SSE transport layer for MCP clients.
- Providing core MCP tools: `list_services`, `get_service_status`, and `get_service_history`.
- Implementing flexible configuration via environment variables and TOML.

## Architecture
- **Transport Layer (`http_server.rs`):** Axum-based server handling `/sse` and `/messages` endpoints.
- **Protocol Layer (`mcp.rs`):** JSON-RPC 2.0 dispatching and tool definition management.
- **Domain Layer (`gatus/`):** API client for Gatus, handling authentication and data retrieval.
- **Settings (`settings.rs`):** Centralized configuration using `config-rs`.

## Core Tools
1. **`list_services`**: Returns a list of all services monitored by Gatus.
2. **`get_service_status`**: Returns the current status (Up/Down/Degraded) and latest results for a specific service.
3. **`get_service_history`**: Returns a list of recent health check results for a specific service.

## Technical Constraints
- Must use Rust 1.8x+.
- Must adhere to the Conductor workflow (TDD, >80% coverage).
- Must use `axum`, `tokio`, `reqwest`, and `serde`.
- Payload optimization: Return only essential fields to the LLM.
