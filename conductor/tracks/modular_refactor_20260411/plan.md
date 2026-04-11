# Implementation Plan - Modular Refactor to Match Rescue-Groups-MCP Shape

## Phase 1: Re-scaffolding & Dependencies
- [x] Task: Update Dependencies
    - [x] Add `moka`, `governor`, `clap` (with subcommands), and `tokio` (with extra features).
- [x] Task: Project Structural Reorganization
    - [x] Create `src/cli.rs`, `src/server.rs`, `src/client.rs`, `src/fmt.rs`.
    - [x] Move existing Gatus logic from `src/gatus/mod.rs` to `src/client.rs`.

## Phase 2: Core Refactoring
- [x] Task: Implement `src/cli.rs`
    - [x] Define subcommands for `stdio`, `http`, and direct tool calls.
- [x] Task: Refactor `src/mcp.rs`
    - [x] Update to latest protocol version and modularize tool dispatching.
- [x] Task: Implement `src/server.rs`
    - [x] Port SSE logic from `src/http_server.rs` and implement Stdio transport.
- [x] Task: Implement `src/fmt.rs`
    - [x] Add Markdown formatting for health check results.

## Phase 3: Final Integration & Verification
- [x] Task: Update `src/main.rs`
    - [x] Connect CLI to server logic.
- [x] Task: Update `Taskfile.yml`
    - [x] Add cross-compilation and advanced testing tasks.
- [x] Task: Conductor - User Manual Verification 'Modular Architecture' (Protocol in workflow.md)
