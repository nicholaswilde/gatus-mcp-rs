# Specification - Targeted Endpoint Status

## Overview
Optimize service history retrieval by using targeted API calls instead of fetching all endpoint statuses and filtering client-side.

## Goals
- Improve token efficiency by reducing the volume of data fetched from Gatus.
- Reduce latency for service-specific queries.

## Implementation Details
- **Endpoint**: `GET /api/v1/endpoints/{key}/statuses`
- **Logic**: If a specific service is requested in `service-history`, use the targeted endpoint.
