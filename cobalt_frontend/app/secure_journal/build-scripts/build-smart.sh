#!/bin/bash
set -e

VERSION=${1:-"0.1.0"}

echo "=== Smart Build for Secure Journal v$VERSION ==="
echo ""

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

echo "Detected: $OS on $ARCH"
echo ""

# Build for native platform
echo "📦 Building for native platform..."
cargo build --release --manifest-path backend/Cargo.toml
cargo build --release --manifest-path frontend/Cargo.toml

# Create dist directories
mkdir -p target/dist
mkdir -p target/releases/v$VERSION

# Package native build
echo ""
echo "📦 Packaging native build..."

if [[ "$OS" == "Linux" ]]; then
    # Linux packaging
    RELEASE_NAME="secure-journal-linux-${ARCH}-v${VERSION}"
    mkdir -p "target/dist/$RELEASE_NAME"
    
    cp target/release/backend "target/dist/$RELEASE_NAME/"
    cp target/release/frontend "target/dist/$RELEASE_NAME/" 2>/dev/null || true
    cp README.md "target/dist/$RELEASE_NAME/" 2>/dev/null || true
    
    # Create tar.gz
    cd target/dist
    tar -czf "../releases/v$VERSION/$RELEASE_NAME.tar.gz" "$RELEASE_NAME/"
    tar -cJf "../releases/v$VERSION/$RELEASE_NAME.tar.xz" "$RELEASE_NAME/"
    cd ../..
    
    echo "✅ Created: $RELEASE_NAME.tar.gz"
    echo "✅ Created: $RELEASE_NAME.tar.xz"
    
elif [[ "$OS" == "Darwin" ]]; then
    # macOS packaging
    RELEASE_NAME="secure-journal-macos-${ARCH}-v${VERSION}"
    mkdir -p "target/dist/$RELEASE_NAME"
    
    cp target/release/backend "target/dist/$RELEASE_NAME/"
    cp target/release/frontend "target/dist/$RELEASE_NAME/" 2>/dev/null || true
    cp README.md "target/dist/$RELEASE_NAME/" 2>/dev/null || true
    
    cd target/dist
    tar -czf "../releases/v$VERSION/$RELEASE_NAME.tar.gz" "$RELEASE_NAME/"
    cd ../..
    
    echo "✅ Created: $RELEASE_NAME.tar.gz"
fi

# Try cross-compilation with 'cross' if available
if command -v cross &> /dev/null; then
    echo ""
    echo "📦 Cross-compilation with 'cross' tool..."
    
    # Try Windows
    if cross build --release --manifest-path backend/Cargo.toml --target x86_64-pc-windows-gnu 2>/dev/null; then
        echo "✅ Windows build successful"
        mkdir -p "target/dist/secure-journal-windows-x64-v$VERSION"
        cp target/x86_64-pc-windows-gnu/release/backend.exe "target/dist/secure-journal-windows-x64-v$VERSION/"
        cd target/dist
        zip -r "../releases/v$VERSION/secure-journal-windows-x64-v$VERSION.zip" "secure-journal-windows-x64-v$VERSION/"
        cd ../..
    else
        echo "⚠️  Windows cross-compilation failed (skipping)"
    fi
else
    echo ""
    echo "ℹ️  'cross' tool not found. Install it for cross-platform builds:"
    echo "   cargo install cross --git https://github.com/cross-rs/cross"
fi

echo ""
echo "======================================"
echo "✅ Build complete!"
echo "======================================"
echo ""
echo "📁 Release files in: target/releases/v$VERSION/"
ls -lh "target/releases/v$VERSION/" 2>/dev/null || echo "   (no files yet)"
echo ""
echo "💡 For full cross-platform builds:"
echo "   1. Use GitHub Actions (automatic)"
echo "   2. Install 'cross': cargo install cross --git https://github.com/cross-rs/cross"
echo "   3. Build on native platforms (Windows for .exe, Linux for .deb, etc.)"