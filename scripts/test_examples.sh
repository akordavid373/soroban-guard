#!/bin/bash

# Test Soroban Security Guard with example contracts

set -e

echo "🧪 Testing Soroban Security Guard with examples..."

# Build the project first
cargo build --release

BINARY="./target/release/soroban-security-guard"
EXAMPLES_DIR="./examples"

echo "📋 Testing vulnerable contract..."
$BINARY scan "$EXAMPLES_DIR/vulnerable_contract.rs" --output json --output-file vulnerable_report.json

echo "📋 Testing safe contract..."
$BINARY scan "$EXAMPLES_DIR/safe_contract.rs" --output json --output-file safe_report.json

echo "📋 Listing available rules..."
$BINARY list-rules

echo "📋 Generating configuration file..."
$BINARY init-config --output test-config.toml

echo "✅ All tests completed!"
echo "📁 Reports generated: vulnerable_report.json, safe_report.json"
