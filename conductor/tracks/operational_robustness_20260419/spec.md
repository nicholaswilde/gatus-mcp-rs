# Specification - Operational Robustness

## Overview
Enhance the reliability and security of the MCP server by supporting Basic Authentication and implementing automatic key sanitization.

## Goals
- Support Gatus instances protected by username/password.
- Simplify endpoint identification for LLMs by handling Gatus's internal key formatting.

## Implementation Details
- **Basic Auth**: 
    - Support `GATUS_USERNAME` and `GATUS_PASSWORD` environment variables.
    - Update `GatusClient` to use Basic Auth when credentials are provided.
- **Key Sanitization**: 
    - Implement a helper to convert "Group Name" and "Service Name" into the internal Gatus key (e.g., `Group---Name_Service---Name`).
