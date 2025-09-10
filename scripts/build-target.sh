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

# Create platform-specific directory name from target
platform_dir=$(echo "$target" | sed 's/unknown-//' | sed 's/pc-//' | sed 's/apple-darwin/macos/' | sed 's/linux-gnu/linux/')
release_dir="release/$platform_dir"

# Create the platform-specific directory
mkdir -p "$release_dir"

# Copy binary to platform directory with standard name (keeping .exe for Windows)
final_binary_name="mcp-serve"
if [[ "$target" == *windows* ]]; then
  final_binary_name="mcp-serve.exe"
fi

cp "target/$target/release/$binary_name" "$release_dir/$final_binary_name"
echo "✓ Built: $platform_dir/$final_binary_name"