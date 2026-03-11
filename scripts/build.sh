#!/bin/bash

# Soroban Security Guard Build Script

set -e

echo "🛡️  Building Soroban Security Guard..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first."
    exit 1
fi

echo "📦 Installing dependencies..."
cargo fetch

echo "🔧 Building project..."
cargo build --release

echo "🧪 Running tests..."
cargo test

echo "✅ Build completed successfully!"
echo "📁 Binary location: target/release/soroban-security-guard"
