# Specification - Implement alert history retrieval tool

## Overview
Add a `get_alert_history` tool to the MCP server. This tool will fetch the latest alerting events from Gatus to help identify when and why notifications were triggered.

## Goals
- Interface with the Gatus alerts endpoint.
- Provide a clear history of transitions between healthy and unhealthy states.

## MCP Tool Definition
- **Name:** `get_alert_history`
- **Description:** Retrieve recent alert events and state transitions.
- **Input Schema:**
    - `limit`: (integer, optional) Maximum number of alerts to return. Default is 5.
