# Specification - Implement uptime calculation and reporting

## Overview
Implement logic to calculate uptime percentages for endpoints based on their health check results. This provides long-term health metrics beyond the current status.

## Goals
- Parse health check results to calculate "success" vs "failure" ratios.
- Support timeframes:
    - Last 24 Hours
    - Last 7 Days
    - Last 30 Days

## MCP Tool Definition
- **Name:** `get_uptime`
- **Description:** Get the uptime percentage for a specific service over a given timeframe.
- **Input Schema:**
    - `service`: (string, required) Name of the service.
    - `timeframe`: (string, optional) "24h", "7d", or "30d". Default is "24h".
