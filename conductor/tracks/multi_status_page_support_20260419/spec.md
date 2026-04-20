# Specification: Multi-Status Page Support

## Goal
Manage and monitor health across multiple Gatus status pages.

## Features
### 1. List Status Pages (`list-status-pages`)
- **Description:** Retrieve all configured status pages.
### 2. Get Page Health (`get-page-health`)
- **Description:** Get aggregated health for a specific status page.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
