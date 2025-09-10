#!/bin/bash
set -euo pipefail

if [ $# -eq 0 ]; then
  echo "Usage: $0 <target>"
  echo "Example: $0 x86_64-unknown-linux-gnu"
  exit 1
fi

target="$1"
echo "Building for $target..."

# Create release directory if it doesn't exist
mkdir -p release

# Add the target if it's not already installed
if ! rustup target list --installed | grep -q "^${target}$"; then
  echo "Installing target $target..."
  rustup target add "$target"
fi

# Build for the specified target using cross
cross build --release --target "$target"

# Determine binary name and extension
binary_name="mcp-serve"
if [[ "$target" == *windows* ]]; then
  binary_name="${binary_name}.exe"
fi

# Create platform-specific binary name from target
platform_binary="mcp-serve-$target"
# Clean up the name for readability
platform_binary=$(echo "$platform_binary" | sed 's/unknown-//' | sed 's/pc-//' | sed 's/apple-darwin/macos/' | sed 's/linux-gnu/linux/')
# Add .exe extension for Windows
if [[ "$target" == *windows* ]]; then
  platform_binary="${platform_binary}.exe"
fi

# Copy binary to release directory
cp "target/$target/release/$binary_name" "release/$platform_binary"
echo "✓ Built: $platform_binary"