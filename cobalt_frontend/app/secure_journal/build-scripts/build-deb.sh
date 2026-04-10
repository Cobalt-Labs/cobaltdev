#!/bin/bash
set -e

VERSION=${1:-"0.1.0"}
ARCH="amd64"

echo "Building .deb package for Secure Journal v$VERSION"

# Install cargo-deb if not present
if ! command -v cargo-deb &> /dev/null; then
    echo "Installing cargo-deb..."
    cargo install cargo-deb
fi

# Build the .deb package
cargo deb --manifest-path backend/Cargo.toml --target x86_64-unknown-linux-gnu

# Copy to releases
RELEASE_DIR="target/releases/v$VERSION"
mkdir -p "$RELEASE_DIR"
cp target/x86_64-unknown-linux-gnu/debian/*.deb "$RELEASE_DIR/"

echo "✅ Debian package created in $RELEASE_DIR"