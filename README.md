# Gatus MCP Server

A Model Context Protocol (MCP) server for [Gatus](https://gatus.io), the automated health check dashboard. This server enables Large Language Models (LLMs) to interact with Gatus APIs to monitor service health, retrieve check history, and diagnose issues.

> [!IMPORTANT]
> This MCP server currently only works with **self-hosted Gatus instances** and is not compatible with the `gatus.io` managed service.

## Features

- **Model Context Protocol (MCP):** Native support for MCP, allowing easy integration with AI tools like Claude Desktop.
- **Service Monitoring:** List all monitored services and their current statuses (UP/DOWN/DEGRADED).
- **System Health Summary:** High-level overview of total, up, down, and degraded endpoint counts.
- **Detailed Diagnostics:** Fetch latest results and history for specific health checks.
- **Multiple Transports:** Support for both Stdio and HTTP (SSE) transport layers.
- **Optimized for LLMs:** Returns "thinned" payloads to conserve token usage while providing high-signal information.
- **Flexible Configuration:** Configure via environment variables, `config.toml`, or CLI flags.

## Installation

### Prerequisites

- Rust 1.8x or higher.
- A running Gatus instance.

### From Source

```bash
git clone https://github.com/nicholaswilde/gatus-mcp-rs.git
cd gatus-mcp-rs
cargo build --release
```

The binary will be available at `target/release/gatus-mcp-rs`.

## Configuration

You can configure the server using environment variables or a `config.toml` file.

### Environment Variables

- `GATUS_API_URL`: The base URL of your Gatus instance (e.g., `http://localhost:8080`).
- `GATUS_API_KEY`: (Optional) Your Gatus API key for authentication.
- `GATUS_SERVER_PORT`: Port for the HTTP server (default: `8080`).
- `LOG_LEVEL`: Logging level (`error`, `warn`, `info`, `debug`, `trace`).

### Example `config.toml`

```toml
[server]
port = 8080
host = "127.0.0.1"

[gatus]
api_url = "http://localhost:8080"
# api_key = "your-api-key"
```

## Usage

### Stdio Mode (Default)

Ideal for integration with desktop LLM clients.

```bash
gatus-mcp-rs stdio
```

### HTTP (SSE) Mode

For remote clients or distributed setups.

```bash
gatus-mcp-rs http --port 8080 --host 0.0.0.0
```

### Command Line Interface

```bash
Usage: gatus-mcp-rs [OPTIONS] [COMMAND]

Commands:
  stdio       Run in stdio mode (default)
  http        Run in HTTP (SSE) mode
  list-tools  List available MCP tools
  call-tool   Call a specific tool directly
  help        Print this message or the help of the given subcommand(s)

Options:
  -u, --gatus-url <GATUS_URL>    Gatus API URL [env: GATUS_API_URL=]
  -k, --api-key <API_KEY>        Gatus API Key [env: GATUS_API_KEY]
  -l, --log-level <LOG_LEVEL>    Log level [default: info]
  -f, --log-format <LOG_FORMAT>  Log format (text, json) [default: text]
  -h, --help                     Print help
  -V, --version                  Print version
```

## MCP Tool Reference

### `manage_services`
Manage and list Gatus monitored services.
- **Arguments:**
  - `action`: `list` (compact markdown table) or `status` (detailed JSON).

### `get_service_info`
Retrieve detailed information or history for a specific service.
- **Arguments:**
  - `service`: Name of the service (e.g., "Authentik").
  - `action`: `details` (current status/latest result) or `history` (recent results).
  - `limit`: (Optional) Number of history records to return (default: 10).

### `get_system_stats`
Get a high-level summary of all monitored services.
- **Arguments:** (none)

## License

This project is licensed under the Apache License, Version 2.0.
