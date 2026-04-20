# Tech Stack - Gatus MCP Server

## Core Language & Runtime
- **Rust (v1.8x+):** The primary programming language, chosen for its safety, performance, and strong ecosystem for building reliable network services.
- **Tokio:** The industry-standard asynchronous runtime for Rust, providing non-blocking I/O and task scheduling.
- **chrono:** A powerful date and time library for Rust, used for parsing timestamps and calculating timeframes for metrics like uptime.

## Web & API Layer
- **Axum:** A fast, ergonomic web framework built on top of the `hyper` crate. Used to implement the SSE (Server-Sent Events) transport layer for MCP.
- **reqwest:** A high-level HTTP client used to communicate with the Gatus API, configured with `rustls-tls` for a lightweight, system-independent security stack.
- **serde & serde_json:** Powerful serialization and deserialization libraries for handling JSON-RPC messages and Gatus API responses.

## Configuration & Management
- **config-rs:** A flexible configuration library for merging settings from TOML files, environment variables, and CLI flags.
- **clap (v4):** A full-featured command-line argument parser used for server bootstrapping and initial configuration.
- **dashmap:** A high-performance, concurrent hash map used for thread-safe session tracking across multiple SSE streams.
- **tokio::sync::broadcast:** Used for efficient, one-to-many notification dispatching of real-time service state changes.
- **moka:** A high-performance concurrent caching library used for caching Gatus API responses and reducing upstream load.
- **governor:** A rate-limiting library used to ensure compliance with Gatus API rate limits and protect the upstream server.

## Data Processing
- **regex:** Used for parsing raw Prometheus metrics from the Gatus /metrics endpoint to identify flapping services and system-wide patterns.

## Logging & Observability
- **tracing:** A framework for instrumenting Rust programs to collect structured, event-based diagnostic information.
- **tracing-subscriber:** Used for formatting and outputting traces to the console or log files.

## Build & CI/CD
- **Cargo:** Rust's built-in package manager and build system.
- **Taskfile.yml:** A task runner (go-task) for standardizing developer workflows (building, testing, linting).
- **profile.release:** Highly optimized release settings (opt-level "z", LTO, and stripped symbols) to minimize binary size.

## Error Handling
- **anyhow:** A flexible error type for high-level application code.
- **thiserror:** Used for defining idiomatic, domain-specific error types where more control is needed.

## Security
- **rustls-tls:** A pure Rust implementation of TLS, used to avoid external dependencies like OpenSSL and simplify cross-platform builds.
- **uuid (v4):** For generating unique session IDs for MCP clients.
