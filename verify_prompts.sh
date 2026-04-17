#!/bin/bash
set -e

# Build the project first
cargo build --quiet

# List Prompts
echo "Testing prompts/list..."
echo '{"jsonrpc": "2.0", "method": "prompts/list", "id": 1}' | ./target/debug/gatus-mcp-rs stdio | grep -q "analyze-outage"
echo '{"jsonrpc": "2.0", "method": "prompts/list", "id": 1}' | ./target/debug/gatus-mcp-rs stdio | grep -q "daily-health-report"
echo "Prompts list test passed."

# Get Analyze Outage Prompt
echo "Testing prompts/get for analyze-outage..."
echo '{"jsonrpc": "2.0", "method": "prompts/get", "params": {"name": "analyze-outage", "arguments": {"id": "test-service"}}, "id": 2}' | ./target/debug/gatus-mcp-rs stdio | grep -q "test-service"
echo "Analyze outage prompt test passed."

# Get Daily Health Report Prompt
echo "Testing prompts/get for daily-health-report..."
echo '{"jsonrpc": "2.0", "method": "prompts/get", "params": {"name": "daily-health-report"}, "id": 3}' | ./target/debug/gatus-mcp-rs stdio | grep -q "system-stats"
echo "Daily health report prompt test passed."

echo "All automatic verification tests passed."
