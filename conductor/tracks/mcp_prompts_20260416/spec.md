# Specification - Implement MCP Prompts

## Overview
Expose common monitoring workflows as MCP Prompts to guide LLMs in diagnosing issues and reporting health.

## Goals
- Provide a standard prompt for outage analysis.
- Provide a standard prompt for daily health reports.

## MCP Prompt Definitions
- **`analyze-outage`**: Instructions to fetch alert history and service history to diagnose a failure.
- **`daily-health-report`**: Instructions to summarize system health and group statuses.
