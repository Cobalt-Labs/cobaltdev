#!/bin/bash
set -e

echo "Building for Android..."

# Install Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

# Set Android NDK path (adjust to your installation)
export ANDROID_NDK_HOME="$HOME/Android/Sdk/ndk/25.2.9519653"

# Build for Android
cargo build --release --target aarch64-linux-android --manifest-path backend/Cargo.toml

echo "Android build complete!"
echo "Note: Additional APK packaging steps required using Android Studio or Gradle"