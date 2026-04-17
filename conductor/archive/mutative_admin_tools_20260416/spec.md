# Specification - Implement Mutative / Administrative Tools

## Overview
Add tools to allow LLMs to trigger actions in Gatus, such as force-checking a service or reloading the configuration.

## Goals
- Enable immediate verification of fixes.
- Support automated configuration management.

## MCP Tool Definitions
- **`trigger_check`**: Force an immediate health check for a specific endpoint.
- **`reload_config`**: Trigger a Gatus configuration reload.
