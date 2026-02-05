#!/bin/bash
set -euo pipefail

# Build pipelines-rs library and RAT WASM UI
# Local-only build (no GitHub Pages deployment)

cd "$(dirname "$0")/.."

echo "Building Rust library..."
cargo build

echo ""
echo "Building RAT WASM UI with trunk..."
cd wasm-ui-rat
trunk build

echo ""
echo "Build complete!"
echo "Run ./scripts/serve-rat.sh to start the server on port 9953"
