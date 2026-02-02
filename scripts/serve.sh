#!/bin/bash
set -euo pipefail

# Serve the WASM UI on port 9952
# Creates a symlink structure to match GitHub Pages path /pipelines-rs/

cd "$(dirname "$0")/.."

# Create a temporary serve directory with the right path structure
SERVE_DIR="$(mktemp -d)"
trap "rm -rf $SERVE_DIR" EXIT

# Create symlink: $SERVE_DIR/pipelines-rs -> docs/
ln -s "$(pwd)/docs" "$SERVE_DIR/pipelines-rs"

echo "Serving pipelines-rs UI at http://localhost:9952/pipelines-rs/"
echo "Press Ctrl+C to stop"
basic-http-server -a 0.0.0.0:9952 "$SERVE_DIR"
