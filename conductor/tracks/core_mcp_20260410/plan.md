# Implementation Plan - Build core Gatus MCP server functionality

## Phase 1: Project Foundation & Scaffolding [checkpoint: 7d6f304]
- [x] Task: Project Initialization [c3ce159]
    - [x] Create `Cargo.toml` with "Gatus Stack" dependencies
    - [x] Set up project structure (`src/`, `tests/`)
    - [x] Configure `Taskfile.yml` for testing, building, and linting
- [x] Task: Configuration Management [ca865f1]
    - [x] Write Tests: Verify `Settings` can be loaded from TOML and Env
    - [x] Implement `src/settings.rs` using `config-rs` and `clap`
- [x] Task: Conductor - User Manual Verification 'Foundation & Scaffolding' (Protocol in workflow.md)

## Phase 2: Transport & Protocol Implementation
- [ ] Task: SSE Transport Layer
    - [ ] Write Tests: Verify `/sse` and `/messages` endpoints respond correctly
    - [ ] Implement `src/http_server.rs` with Axum and session management
- [ ] Task: MCP Protocol Handler
    - [ ] Write Tests: Verify JSON-RPC 2.0 dispatching for unknown tools
    - [ ] Implement `src/mcp.rs` for core JSON-RPC logic
- [ ] Task: Conductor - User Manual Verification 'Transport & Protocol' (Protocol in workflow.md)

## Phase 3: Gatus Integration (Core Tools)
- [ ] Task: Gatus API Client
    - [ ] Write Tests: Mock Gatus API and verify client handles responses/errors
    - [ ] Implement `src/gatus/mod.rs` for communication with Gatus
- [ ] Task: Core Tool: `list_services`
    - [ ] Write Tests: Verify `list_services` returns thinned service definitions
    - [ ] Implement `list_services` tool in `src/mcp.rs`
- [ ] Task: Core Tool: `get_service_status`
    - [ ] Write Tests: Verify `get_service_status` returns correct status for a given service
    - [ ] Implement `get_service_status` tool in `src/mcp.rs`
- [ ] Task: Core Tool: `get_service_history`
    - [ ] Write Tests: Verify `get_service_history` returns limited health check results
    - [ ] Implement `get_service_history` tool in `src/mcp.rs`
- [ ] Task: Conductor - User Manual Verification 'Gatus Integration' (Protocol in workflow.md)

## Phase 4: Final Refinement
- [ ] Task: Error Handling & Logging
    - [ ] Write Tests: Verify error responses for failed API calls
    - [ ] Refine `tracing` instrumentation across all modules
- [ ] Task: README & Documentation
    - [ ] Document all MCP tools and their usage
    - [ ] Finalize installation and setup instructions
- [ ] Task: Conductor - User Manual Verification 'Final Refinement' (Protocol in workflow.md)
