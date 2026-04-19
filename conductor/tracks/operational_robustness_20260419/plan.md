# Implementation Plan - Operational Robustness

## Phase 1: Security & Auth
- [x] Task: Add Basic Auth support to `GatusClient`. [81431ce]
- [x] Task: Update `Settings` to load username/password from env. [81431ce]
- [x] Task: Add unit tests for Basic Auth headers. [8bde185]

## Phase 2: Key Management
- [ ] Task: Implement `GatusClient::sanitize_key` helper.
- [ ] Task: Update MCP tools to use the sanitize helper if a key is not clearly provided.
- [ ] Task: Add unit tests for key sanitization logic.
