#!/bin/bash
set -e

VERSION=${1:-"0.1.0"}
echo "Packaging Secure Journal v$VERSION"

DIST_DIR="target/dist"
RELEASE_DIR="target/releases/v$VERSION"

mkdir -p "$RELEASE_DIR"

# Package Windows
echo "Packaging Windows..."
mkdir -p "$DIST_DIR/secure-journal-windows"
cp target/x86_64-pc-windows-msvc/release/backend.exe "$DIST_DIR/secure-journal-windows/"
cp target/x86_64-pc-windows-msvc/release/frontend.exe "$DIST_DIR/secure-journal-windows/" 2>/dev/null || true
cp README.md "$DIST_DIR/secure-journal-windows/"
cd "$DIST_DIR" && zip -r "$RELEASE_DIR/secure-journal-windows-x64-v$VERSION.zip" secure-journal-windows/
cd - > /dev/null

# Package Linux (tar.gz)
echo "Packaging Linux (tar.gz)..."
mkdir -p "$DIST_DIR/secure-journal-linux"
cp target/x86_64-unknown-linux-gnu/release/backend "$DIST_DIR/secure-journal-linux/"
cp target/x86_64-unknown-linux-gnu/release/frontend "$DIST_DIR/secure-journal-linux/" 2>/dev/null || true
cp README.md "$DIST_DIR/secure-journal-linux/"
chmod +x "$DIST_DIR/secure-journal-linux/backend"
chmod +x "$DIST_DIR/secure-journal-linux/frontend" 2>/dev/null || true
cd "$DIST_DIR" && tar -czf "$RELEASE_DIR/secure-journal-linux-x64-v$VERSION.tar.gz" secure-journal-linux/
cd - > /dev/null

# Package Linux (tar.xz)
echo "Packaging Linux (tar.xz)..."
cd "$DIST_DIR" && tar -cJf "$RELEASE_DIR/secure-journal-linux-x64-v$VERSION.tar.xz" secure-journal-linux/
cd - > /dev/null

# Package ARM64 Linux
echo "Packaging ARM64 Linux..."
mkdir -p "$DIST_DIR/secure-journal-linux-arm64"
cp target/aarch64-unknown-linux-gnu/release/backend "$DIST_DIR/secure-journal-linux-arm64/" 2>/dev/null || true
cp target/aarch64-unknown-linux-gnu/release/frontend "$DIST_DIR/secure-journal-linux-arm64/" 2>/dev/null || true
cp README.md "$DIST_DIR/secure-journal-linux-arm64/"
cd "$DIST_DIR" && tar -czf "$RELEASE_DIR/secure-journal-linux-arm64-v$VERSION.tar.gz" secure-journal-linux-arm64/
cd - > /dev/null

echo ""
echo "✅ All packages created in $RELEASE_DIR"
ls -lh "$RELEASE_DIR"