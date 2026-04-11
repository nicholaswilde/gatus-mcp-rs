# Specification - Implement system statistics tool

## Overview
Add a `get_system_stats` tool to the MCP server. This tool will query the Gatus API and aggregate the results to provide a high-level summary of the monitored system's health.

## Goals
- Aggregate data from `/api/v1/endpoints/statuses`.
- Provide counts for:
    - Total Endpoints
    - Endpoints UP
    - Endpoints DOWN
    - Endpoints DEGRADED

## MCP Tool Definition
- **Name:** `get_system_stats`
- **Description:** Get a high-level summary of all monitored services (total, up, down).
- **Input Schema:** Empty object `{}`.
