# Implementation Plan - Operational Robustness

## Phase 1: Security & Auth [checkpoint: e2d7b7a]
- [x] Task: Add Basic Auth support to `GatusClient`. [4f7f06a]
- [x] Task: Update `Settings` to load username/password from env. [4f7f06a]
- [x] Task: Add unit tests for Basic Auth headers. [4f7f06a]

## Phase 2: Key Management
- [x] Task: Implement `GatusClient::sanitize_key` helper. [0fbff45]
- [x] Task: Update MCP tools to use the sanitize helper if a key is not clearly provided. [0fbff45]
- [x] Task: Add unit tests for key sanitization logic. [0fbff45]
