# Specification - Implement MCP Resources

## Overview
Expose static or slowly-changing Gatus data as MCP Resources for direct context attachment.

## Goals
- Expose the Gatus configuration as a resource.
- Expose the high-level dashboard status as a resource.

## MCP Resource Definitions
- **`gatus://system/config`**: The active Gatus YAML/TOML configuration.
- **`gatus://dashboard/status`**: A summary of current endpoint statuses.
