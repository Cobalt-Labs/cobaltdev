#!/bin/bash
set -e

echo "Building for native platform..."

# Build backend
cargo build --release --manifest-path backend/Cargo.toml

# Build frontend
cargo build --release --manifest-path frontend/Cargo.toml

# Create distribution directory
mkdir -p target/dist/native

# Copy binaries
cp target/release/backend target/dist/native/
cp target/release/frontend target/dist/native/ 2>/dev/null || true

echo "✅ Native build complete! Files in target/dist/native"