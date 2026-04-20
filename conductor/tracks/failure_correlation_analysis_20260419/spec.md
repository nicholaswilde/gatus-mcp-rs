# Specification: Failure Correlation Analysis

## Goal
Identify regional or infrastructure-wide failure patterns by correlating data across groups.

## Features
### 1. Get Group Correlation (`get-group-correlation`)
- **Description:** Identify patterns in failures across different groups and regions.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
