# Specification: Service Drift & Jitter Analysis

## Goal
Analyze response jitter and configuration drift for predictive failure detection.

## Features
### 1. Get Response Jitter (`get-response-jitter`)
- **Description:** Calculate response time variance for an endpoint.
### 2. Check Config Drift (`check-config-drift`)
- **Description:** Compare current running config with desired state.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
