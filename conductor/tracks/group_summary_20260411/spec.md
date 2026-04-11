# Specification - Implement group summary tool

## Overview
Add a `get_group_summary` tool to the MCP server. This tool allows LLMs to focus on a specific group of services (e.g., "Media", "DNS") and retrieve their current status.

## Goals
- Filter endpoints by the `group` field.
- Provide a summary of all services within the specified group.

## MCP Tool Definition
- **Name:** `get_group_summary`
- **Description:** Get the health status of all endpoints within a specific group.
- **Input Schema:**
    - `group`: (string, required) The name of the group to summarize.
