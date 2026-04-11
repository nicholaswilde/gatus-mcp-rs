# Specification - Implement configuration retrieval tool

## Overview
Add a `get_config` tool to retrieve and display the current Gatus configuration. This helps LLMs understand the monitoring parameters (intervals, conditions, etc.).

## Goals
- Interface with the Gatus config endpoint (if available) or expose known configuration from the server.
- Return a summarized view of the monitoring setup.

## MCP Tool Definition
- **Name:** `get_config`
- **Description:** Retrieve the current Gatus monitoring configuration.
- **Input Schema:** Empty object `{}`.
