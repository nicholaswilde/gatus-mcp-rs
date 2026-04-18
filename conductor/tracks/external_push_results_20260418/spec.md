# Specification - External Endpoint Push

## Overview
Implement support for pushing health check results from external systems directly to Gatus via the API.

## Goals
- Allow LLMs and external asynchronous tasks to report their status to the Gatus dashboard.
- Enable monitoring of "push-based" endpoints.

## Implementation Details
- **Endpoint**: `POST /api/v1/endpoints/{key}/results`
- **Payload**: Gatus `Result` object (timestamp, success, errors, etc.)
- **Tool Integration**: Add `push-result` action to the mutative toolset.
