# Specification - Implement Proactive Notifications (SSE)

## Overview
Leverage the SSE transport layer to push real-time notifications to MCP clients when service states change.

## Goals
- Reduce LLM polling requirements.
- Provide immediate awareness of critical failures.

## Notification Types
- **Service Down**: Triggered when a service transitions to `DOWN`.
- **Service Recovered**: Triggered when a service transitions back to `UP`.
