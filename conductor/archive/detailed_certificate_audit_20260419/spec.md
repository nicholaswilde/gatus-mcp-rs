# Specification: Detailed Certificate Audit

## Goal
Provide deep insights into SSL/TLS certificates for monitored endpoints.

## Features
### 1. Get Certificate Audit (`get-certificate-audit`)
- **Description:** Retrieve detailed certificate metadata including Issuer, SANs, Algorithm, and more.

## Technical Considerations
- Ensure all new tools follow the existing "thinned" payload pattern to conserve tokens.
- Maintain compatibility with self-hosted Gatus instances.
- Update `GatusClient` to support the required API calls for each feature.
