# Specification - Deep-Dive Diagnostics

## Overview
Enable LLMs to perform root cause analysis by providing access to raw health check results and detailed endpoint configurations.

## Goals
- Allow LLMs to inspect full response bodies and headers on failure.
- Provide access to detailed monitoring conditions for each endpoint.

## Implementation Details
- **Raw Results**: 
    - Add a `get-raw-results` action to `get_metrics` to return the last N full JSON results for an endpoint.
- **Detailed Config**: 
    - Investigation of the `/api/v1/endpoints/{key}/statuses` endpoint to extract detailed condition rules and pass/fail history for each condition.
