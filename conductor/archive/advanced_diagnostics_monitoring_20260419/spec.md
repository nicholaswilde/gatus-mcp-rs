# Specification: Advanced Diagnostics and Monitoring Enhancements

## Goal
Enhance the Gatus MCP server with advanced diagnostic capabilities to improve incident response, proactive maintenance, and performance monitoring for SREs and developers.

## Features

### 1. Proactive Certificate Monitoring (`list-expiring-certificates`)
- **Description:** List all monitored endpoints with SSL certificates expiring within 30 days.
- **Output:** Markdown table with service name, group, expiration date, and days remaining.

### 2. Advanced Status Filtering
- **Description:** Enhance `list-services` to support filtering by status (e.g., `down`, `degraded`).
- **Output:** Filtered list of endpoints.

### 3. High-Signal Failure Analysis (`get-failure-summary`)
- **Description:** Extract and summarize exactly which conditions failed for a given service.
- **Output:** Concise list of failing vs. passing conditions for the latest result.

### 4. Latency Regression Detection (`compare-performance`)
- **Description:** Compare current response times against historical averages to detect regressions.
- **Output:** Percentage change in latency between the last hour and the last 7 days.

### 5. Group Health Aggregation (`get-group-stats`)
- **Description:** Calculate and return health percentages for all endpoints within a specific group.
- **Output:** Percentage of endpoints in UP, DOWN, and DEGRADED states per group.

### 6. Notification Event Correlation
- **Description:** Correlate endpoint check failures with subsequent notification events in the history.
- **Output:** Timeline showing the sequence from failure to alert delivery.

### 7. Dynamic Maintenance Trigger (`set-maintenance`)
- **Description:** Provide a tool to programmatically set maintenance windows (requires appropriate permissions and local config access).
- **Output:** Confirmation of maintenance window activation.

### 8. Resource Utilization Insights
- **Description:** Expose metrics from the `/metrics` endpoint to identify flapping services and system-wide patterns.
- **Output:** Summary of services with the highest state transition counts.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
