#!/bin/bash
set -euo pipefail

# Build the WASM UI for pipelines-rs

cd "$(dirname "$0")/.."

echo "Building WASM UI..."
cd wasm-ui
wasm-pack build --target web --out-dir pkg

# Copy index.html to pkg
cp index.html pkg/

echo ""
echo "Build complete! To run:"
echo "  cd wasm-ui/pkg && python3 -m http.server 9952"
echo "  Then open http://localhost:9952"
