# Specification - Performance Visualization

## Overview
Expose Gatus response time badges and charts via the MCP server to allow LLMs to analyze performance trends visually.

## Goals
- Provide visual indicators of service latency.
- Allow LLMs to embed performance charts in reports.

## Implementation Details
- **Endpoints**:
    - `GET /api/v1/endpoints/{key}/response-times/{duration}/badge.svg`
    - `GET /api/v1/endpoints/{key}/response-times/{duration}/chart.svg`
- **Tool Integration**: 
    - Update `get_metrics` with `get-latency-badge` and `get-latency-chart` actions.
    - Return Markdown links to the SVG assets.
