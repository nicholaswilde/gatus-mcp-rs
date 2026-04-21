# Specification: Advanced Alerting & Verification

## Goal
Expose alerting rules and provide a mechanism to test notification delivery.

## Features
### 1. Get Alert Rules (`get-alert-rules`)
- **Description:** Retrieve the configured alerting rules.
### 2. Test Alert Notification (`test-alert-notification`)
- **Description:** Trigger a test notification to verify integration.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
