#!/bin/bash
set -euo pipefail

# Create release directory
mkdir -p release

# Build for multiple targets
targets=(
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-msvc"
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
)

echo "Building release binaries for ${#targets[@]} targets..."
echo ""

# Get the directory where this script is located
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for target in "${targets[@]}"; do
  "$script_dir/build-target.sh" "$target"
done

# List all built binaries
echo ""
echo "Built release binaries:"
ls -la release/