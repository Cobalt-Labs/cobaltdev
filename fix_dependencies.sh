#!/bin/bash
# fix_dependencies.sh

cd /Users/ibrahimhaji/code/cobaltdev

echo "🔧 Fixing dependency conflicts..."

# Remove all Cargo.lock files
find . -name "Cargo.lock" -type f -delete

# Clean all target directories
find . -name "target" -type d -exec rm -rf {} + 2>/dev/null

# Update the root Cargo.toml with unified versions
echo "✅ Cleaned build artifacts"
echo "📦 Now run: cargo update -p sqlx"
