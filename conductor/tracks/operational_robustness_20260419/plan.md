# Implementation Plan - Operational Robustness

## Phase 1: Security & Auth
- [x] Task: Add Basic Auth support to `GatusClient`. [4f7f06a]
- [x] Task: Update `Settings` to load username/password from env. [4f7f06a]
- [x] Task: Add unit tests for Basic Auth headers. [4f7f06a]

## Phase 2: Key Management
- [ ] Task: Implement `GatusClient::sanitize_key` helper.
- [ ] Task: Update MCP tools to use the sanitize helper if a key is not clearly provided.
- [ ] Task: Add unit tests for key sanitization logic.
