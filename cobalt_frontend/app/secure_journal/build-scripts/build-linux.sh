#!/bin/bash
set -e

echo "Building for Linux..."

# Build backend
cargo build --release --target x86_64-unknown-linux-gnu --manifest-path backend/Cargo.toml

# Build frontend
cargo build --release --target x86_64-unknown-linux-gnu --manifest-path frontend/Cargo.toml

# Create distribution directory
mkdir -p target/dist/linux

# Copy binaries
cp target/x86_64-unknown-linux-gnu/release/backend target/dist/linux/
cp target/x86_64-unknown-linux-gnu/release/frontend target/dist/linux/ 2>/dev/null || true

echo "Linux build complete! Files in target/dist/linux"