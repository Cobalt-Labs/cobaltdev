#!/bin/bash
set -e

echo "=== Building Secure Journal for All Platforms ==="

# Install required targets
echo "Installing required Rust targets..."
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Create base dist directory
mkdir -p target/dist

# Backend builds
echo ""
echo "Building Backend..."
cargo build --release --manifest-path backend/Cargo.toml --target x86_64-pc-windows-msvc
cargo build --release --manifest-path backend/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --release --manifest-path backend/Cargo.toml --target aarch64-unknown-linux-gnu

# Frontend builds
echo ""
echo "Building Frontend..."
cargo build --release --manifest-path frontend/Cargo.toml --target x86_64-pc-windows-msvc
cargo build --release --manifest-path frontend/Cargo.toml --target x86_64-unknown-linux-gnu
cargo build --release --manifest-path frontend/Cargo.toml --target aarch64-unknown-linux-gnu

echo ""
echo "All builds complete!"