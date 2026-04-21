# Initial Concept

1. Architectural Overview
  The server follows a decoupled architecture, separating transport, protocol handling, and domain logic.

   * Transport Layer (src/http_server.rs): Uses Axum to implement the MCP-over-SSE (Server-Sent Events) standard. It
     manages session IDs and routes incoming JSON-RPC messages to the correct active stream.
   * Protocol Layer (src/mcp.rs): Implements the core MCP/JSON-RPC logic. It handles tool discovery (tools/list),
     resource management, and tool execution dispatching.
   * Domain Layer (src/client.rs): Contains the API client logic for the target service (Gatus).
   * Configuration (src/settings.rs): Uses the config crate to merge settings from config.toml, environment
     variables, and CLI flags via clap.

  2. Core Dependencies (The "Gatus Stack")
  Copy these into your Cargo.toml for a robust foundation:

    1 [dependencies]
    2 tokio = { version = "1.28", features = ["full"] }
    3 axum = { version = "0.8", features = ["macros"] }
    4 serde = { version = "1.0", features = ["derive"] }
    5 serde_json = "1.0"
    6 reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
    7 anyhow = "1.0"
    8 clap = { version = "4.3", features = ["derive", "env"] }
    9 config = { version = "0.15.19", features = ["toml"] }
   10 tracing = "0.1"
   11 tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
   12 dashmap = "6.1"         # Critical for thread-safe SSE session tracking
   13 uuid = { version = "1.0", features = ["v4"] }
   14 thiserror = "2.0"       # For idiomatic error handling

  3. Advanced Diagnostic Features
  The server provides advanced diagnostic capabilities for SREs and developers:
   * Proactive Certificate Monitoring: Detect and list SSL certificates expiring soon.
   * High-Signal Failure Analysis: Pinpoint exactly which conditions failed for an endpoint.
   * Latency Regression Detection: Compare recent performance against historical baselines.
   * Group Health Aggregation: Summarize health status at the group level.
   * Notification Event Correlation: Correlate health results with alert events in a chronological timeline.
   * Resource Utilization Insights: Identify flapping services using low-level metrics.

  4. Implementation Patterns to Mimic

  Tool Definition Strategy
  Instead of one massive list, proxmox-mcp-rs uses granular methods to group tool definitions. This keeps the schema
  manageable and token-efficient.

   1 // Pattern from src/mcp.rs
   2 fn get_tool_definitions(&self) -> Vec<Value> {
   3     let mut tools = Vec::new();
   4     tools.extend(self.tool_defs_health_checks()); // e.g. list_results
   5     tools.extend(self.tool_defs_configuration()); // e.g. reload_config
   6     tools
   7 }

  JSON-RPC Dispatching
  Use a match statement on the method name for fast, type-safe tool execution.

   1 pub async fn call_tool(&self, name: &str, args: &Value) -> Result<Value> {
   2     match name {
   3         "get_status" => self.handle_get_status(args).await,
   4         "check_service" => self.handle_check_service(args).await,
   5         _ => anyhow::bail!("Tool not found"),
   6     }
   7 }

  Payload Optimization
  The project includes a mcp_optimization strategy (found in conductor/archive/) that focuses on "Thinning" response
  payloads. When building your Gatus server, only return fields that an LLM actually needs to make decisions (e.g.,
  status, latency, last_error) rather than the full raw JSON response.

  5. Build & CI Best Practices
   * Taskfile.yml: The project uses go-task for common workflows (test, build, lint).
   * Release Profile: The Cargo.toml includes a highly optimized [profile.release] (using opt-level = "z", lto =
     true, and strip = true) to ensure the resulting binary is small and fast—ideal for deployment in containers or
     alongside Gatus.
   * Cross-Platform: Uses rustls-tls in reqwest to avoid openssl system dependencies, making Docker builds much
     simpler.

  6. Project Structure Reference

   1 .
   2 ├── Cargo.toml
   3 ├── src/
   4 │   ├── main.rs          # CLI entry & server bootstrap
   5 │   ├── mcp.rs           # JSON-RPC & Tool logic
   6 │   ├── http_server.rs   # SSE/Axum transport implementation
   7 │   ├── settings.rs      # Config & Env handling
   8 │   └── gatus/           # Your Gatus API client (mimics src/proxmox/)
   9 └── tests/               # Integration tests (crucial for MCP)

---

# Product Definition - Gatus MCP Server

## Vision
The Gatus MCP Server aims to bridge the gap between Large Language Models (LLMs) and the Gatus health monitoring tool. By implementing the Model Context Protocol (MCP), this server enables LLMs to interact directly with Gatus APIs, allowing them to query service statuses, investigate health check failures, and potentially trigger configuration reloads or manual checks. This empowers AI-driven operations and troubleshooting workflows.

## Target Users
- **DevOps Engineers:** Who use LLMs to assist in monitoring and incident response.
- **SREs:** Who need quick, automated access to service health data during on-call rotations.
- **Developers:** Building AI agents that need to aware of the system's operational health.

## Core Goals
1. **Real-time Health Discovery:** Provide a seamless interface for LLMs to list all monitored services and their current statuses.
2. **Deep-Dive Diagnostics:** Enable LLMs to retrieve detailed results and history for specific health checks to diagnose intermittent issues.
3. **Optimized Information Flow:** Implement "thin" response payloads to ensure LLMs receive only the most relevant data, staying within token limits and maintaining focus.
4. **Configuration Retrieval:** Empower LLMs to understand the monitoring setup (intervals, conditions, groups) to better interpret health check failures.
5. **Operational Control:** Provide tools for basic administrative tasks like reloading the Gatus configuration.
6. **Alerting & Verification:** Expose configured alerting rules and provide mechanisms to test notification delivery to ensure reliable incident response integrations.

## Key Features
- **SSE Transport Layer:** Implements the MCP-over-SSE standard using Axum for reliable, long-lived connections.
- **Dynamic Tool Discovery:** Automatically exposes Gatus API capabilities as MCP tools.
- **Standardized Workflows (MCP Prompts):** Provides pre-defined prompts (e.g., `analyze-outage`, `daily-health-report`) to guide LLMs through complex diagnostic and reporting tasks.
- **Proactive Real-time Notifications:** Pushes immediate service state changes (UP/DOWN) to MCP clients over SSE, reducing the need for LLM polling.
- **Direct Context Attachment (MCP Resources):** Exposes static or slowly-changing Gatus data (e.g., configuration, dashboard status) as resources that LLMs can attach directly to their context.
- **Operational Control (Mutative Tools):** Allows LLMs to trigger actions in Gatus, such as forcing an immediate health check (`trigger_check`), reloading the configuration (`reload_config`), and testing alert notifications (`test_alert`).
- **Robust Authentication:** Supports both API Key (Bearer token) and Basic Authentication (username/password), ensuring compatibility with various self-hosted Gatus security configurations.
- **Automatic Key Mapping:** Implements intelligent key sanitization, automatically mapping human-readable service and group names to Gatus's internal hyphenated key format.
- **External Result Pushing:** Enables LLMs and external systems to push health check results directly to Gatus via the `push_result` tool, facilitating monitoring of asynchronous or push-based tasks.
- **Status & Performance Visualization:** Surface Gatus health status and response-time badges, as well as latency charts (SVG) via the `get_metrics` tool, allowing LLMs to embed visual indicators and performance trends in reports and notifications.
- **Endpoint Lifecycle Management:** Enables programmatic creation, modification, and deletion of Gatus endpoints via the `manage_endpoints` tool, allowing LLMs to manage monitoring targets dynamically.
- **Consolidated Toolset:** Replaces granular tools with a streamlined set of dynamic, parameterized tools (`manage_resources`, `get_metrics`) to improve token efficiency and simplify tool discovery.
- **Dynamic Resource Management:** A single tool (`manage_resources`) for listing services, groups, endpoints, suites, and checking instance health, configuration, or suite-level health aggregation.
- **Enhanced Diagnostics & SSL Tracking:** Provides deep-dive troubleshooting data including response body snippets on failure and proactive SSL certificate expiration tracking.
- **Deep-Dive SSL/TLS Metadata:** Detailed certificate auditing via the `get_metrics` tool, exposing issuer, algorithm, and SANs for comprehensive security reviews.
- **Comprehensive Metrics Retrieval:** A single tool (`get_metrics`) for retrieving system stats, service details, history (optimized with targeted API calls), raw result data (non-truncated), uptime, response times, and alert history.
- **Advanced Diagnostic Capabilities:** Includes high-signal failure analysis, latency regression detection, group health aggregation, and notification event correlation.
- **One-Turn Root Cause Analysis:** Aggregates failure summaries, recent results, and alert events into a single "Diagnostic Bundle" via the `get_metrics` tool, enabling LLMs to diagnose complex issues in a single interaction.
- **High-Signal Payloads:** Implements "thinning" of tool responses to ensure only high-signal data is returned, minimizing token consumption.
- **Thread-Safe Session Management:** Uses DashMap for efficient, concurrent handling of multiple MCP clients.
- **Environment-Driven Configuration:** Flexible setup via `config.toml`, environment variables, and CLI flags.

## Success Metrics
- **Seamless Integration:** LLMs can successfully discover and execute all Gatus-related tools.
- **Low Latency:** Minimal overhead in proxying requests between the LLM and the Gatus API.
- **High Signal-to-Noise:** Tool outputs are concise and actionable, leading to efficient problem resolution by AI agents.
