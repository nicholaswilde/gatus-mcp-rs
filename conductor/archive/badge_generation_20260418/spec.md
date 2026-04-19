# Specification - Badge Generation

## Overview
Surface Gatus status badges (SVG) via the MCP server to allow LLMs to embed health indicators in reports and notifications.

## Goals
- Provide easy access to visual health indicators for services.
- Enable "rich" status reporting in generated documentation.

## Implementation Details
- **Endpoints**:
    - `GET /api/v1/endpoints/{key}/health/badge.svg`
    - `GET /api/v1/endpoints/{key}/uptimes/{duration}/badge.svg`
- **Tool Integration**: Add `get-badge` action to `get_metrics` returning the Markdown link to the badge.
