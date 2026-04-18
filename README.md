[![task](https://img.shields.io/badge/Task-Enabled-brightgreen?style=for-the-badge&logo=task&logoColor=white)](https://taskfile.dev/#/)

# Gatus MCP Server (Rust) :robot:

> [!IMPORTANT]
> This project is currently in active development (v0.1.0) and is **not production-ready**. Features may change, and breaking changes may occur without notice.

A Model Context Protocol (MCP) server for [Gatus](https://gatus.io), the automated health check dashboard. This server enables Large Language Models (LLMs) to interact with Gatus APIs to monitor service health, retrieve check history, and diagnose issues.

> [!IMPORTANT]
> This MCP server currently only works with **self-hosted Gatus instances** and is not compatible with the `gatus.io` managed service.

## :sparkles: Features

- **Model Context Protocol (MCP):** Native support for MCP, allowing easy integration with AI tools like Claude Desktop.
- **Service Monitoring:** List all monitored services and their current statuses (UP/DOWN/DEGRADED).
- **System Health Summary:** High-level overview of total, up, down, and degraded endpoint counts.
- **Detailed Diagnostics:** Fetch latest results, history (optimized with targeted API calls), and granular performance metrics for specific health checks.
- **Alert & State Transitions:** Retrieve chronological alert history to identify incident root causes.
- **Uptime Calculation:** Calculate success vs. failure ratios over 24h, 7d, and 30d timeframes.
- **Configuration Retrieval:** Retrieve the effective Gatus monitoring configuration (conditions, names, groups).
- **Multiple Transports:** Support for both Stdio and HTTP (SSE) transport layers.
- **Optimized for LLMs:** Returns "thinned" payloads to conserve token usage while providing high-signal information.
- **Flexible Configuration:** Configure via environment variables, `config.toml`, or CLI flags.

## :hammer_and_wrench: Installation

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

## :gear: Configuration

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

## :rocket: Usage

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

### `manage_resources`
Discover and manage Gatus resources and instance state.
- **Arguments:**
  - `action`: `list-services`, `list-groups`, `list-endpoints`, `get-config`, or `get-health`.
  - `id`: (Optional) Identifier (e.g., group name for `list-endpoints`).

### `get_metrics`
Retrieve status, metrics, and history for services and endpoints.
- **Arguments:**
  - `action`: `system-stats`, `service-details`, `service-history` (optimized), `group-summary`, `uptime`, `uptime-granular`, `response-time`, or `alert-history`.
  - `id`: (Optional) Identifier (e.g., service name for `service-details`, group name for `group-summary`, or endpoint key for `service-history`, `uptime-granular` and `response-time`).
  - `limit`: (Optional) Maximum number of results for history actions (default: 10 for `service-history`, 5 for `alert-history`).
  - `timeframe`: (Optional) `1h`, `24h`, `7d`, or `30d` (default: `24h`) for `uptime`, `uptime-granular` and `response-time`.

### `trigger_check`
Force an immediate health check for a specific endpoint.
- **Arguments:**
  - `id`: The service name or endpoint key to check.

### `reload_config`
Trigger a Gatus configuration reload.
- **Arguments:** None.

## :handshake: Contributing

Contributions are welcome! Please follow standard Rust coding conventions and ensure all tests pass (`task test:ci`) before submitting features.

## :balance_scale: License

[Apache License 2.0](LICENSE)

## :writing_hand: Author

This project was started in 2026 by [Nicholas Wilde](https://github.com/nicholaswilde/).
