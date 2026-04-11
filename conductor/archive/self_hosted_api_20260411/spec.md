# Track Specification: Self-Hosted API Enhancements

## Goal
Implement additional self-hosted API functions from Gatus as MCP tools to provide more granular data retrieval and operational insight.

## Targeted API Endpoints
- `GET /api/v1/endpoints/{key}/uptimes/{duration}`: Retrieve uptime statistics for a specific endpoint and duration.
- `GET /api/v1/endpoints/{key}/response-times/{duration}`: Retrieve response time statistics for a specific endpoint and duration.
- `GET /health`: Retrieve the health status of the Gatus instance itself.

## New MCP Tools
1. `get_endpoint_stats`: Retrieves uptime and/or response time statistics for a specific endpoint key and duration.
2. `get_instance_health`: Retrieves the health status of the Gatus instance.

## Technical Details
- **Endpoint Key:** Generated as `{group}_{name}` with special characters replaced by `-`.
- **Duration:** Supported values are `1h`, `24h`, `7d`, `30d`.
- **Payload Optimization:** Return summarized statistics rather than full raw data to remain token-efficient.
