# Implementation Plan - Implement Proactive Notifications (SSE)

## Phase 1: Polling Logic [checkpoint: aca36d9]
- [x] Task: Implement a background polling task in `src/server.rs`. (aca36d9)
- [x] Task: Track state changes between polling intervals. (aca36d9)

## Phase 2: SSE Push [checkpoint: aca36d9]
- [x] Task: Integrate state change detection with the Axum SSE stream. (aca36d9)
- [x] Task: Implement MCP notification schema. (aca36d9)
