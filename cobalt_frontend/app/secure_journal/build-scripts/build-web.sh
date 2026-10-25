#!/bin/bash

# build-web.sh - Build for web
echo "🌐 Building for Web..."
cd frontend
dx serve --platform web --port 8080

# build-desktop.sh - Build for desktop
echo "🖥️  Building for Desktop..."
cd frontend
dx serve --platform desktop

# build-web-release.sh - Release build for web
echo "🌐 Building Web Release..."
cd frontend
dx build --platform web --release

# build-desktop-release.sh - Release build for desktop
echo "🖥️  Building Desktop Release..."
cd frontend
dx build --platform desktop --release

# run-desktop.sh - Run desktop app directly with cargo
echo "🖥️  Running Desktop App..."
cd frontend
cargo run --features desktop