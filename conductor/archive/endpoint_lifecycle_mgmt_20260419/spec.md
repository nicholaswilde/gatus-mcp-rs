# Specification: Endpoint Lifecycle Management

## Goal
Enable programmatic management of Gatus endpoints, including creation, updates, and deletion.

## Features
### 1. Create Endpoint (`create-endpoint`)
- **Description:** Programmatically add a new endpoint to Gatus.
### 2. Update Endpoint (`update-endpoint`)
- **Description:** Modify an existing endpoint configuration.
### 3. Delete Endpoint (`delete-endpoint`)
- **Description:** Remove an endpoint from Gatus monitoring.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
