# Specification: Diagnostic Bundle Orchestrator

## Goal
Provide a comprehensive diagnostic bundle that aggregates various data points for one-turn root cause analysis.

## Features
### 1. Get Diagnostic Bundle (`get-diagnostic-bundle`)
- **Description:** Aggregates raw results, failure summary, and alerts into one response.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
