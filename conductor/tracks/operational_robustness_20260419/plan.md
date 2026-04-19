# Implementation Plan - Operational Robustness

## Phase 1: Security & Auth
- [ ] Task: Add Basic Auth support to `GatusClient`.
- [ ] Task: Update `Settings` to load username/password from env.
- [ ] Task: Add unit tests for Basic Auth headers.

## Phase 2: Key Management
- [ ] Task: Implement `GatusClient::sanitize_key` helper.
- [ ] Task: Update MCP tools to use the sanitize helper if a key is not clearly provided.
- [ ] Task: Add unit tests for key sanitization logic.
